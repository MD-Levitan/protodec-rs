use crate::proto::error::*;
use crate::proto::field::*;
use crate::proto::message::*;

pub trait Parser {
    fn deserialize(&self, into: &[u8]) -> Result<Message>;
}

const SimpleFieldsOrder: &'static [FieldType] = &[
    FieldType::Int32,
    FieldType::Int64,
    FieldType::UInt32,
    FieldType::UInt64,
    FieldType::SInt32,
    FieldType::SInt64,
    FieldType::Bool,
    FieldType::Fixed64,
    FieldType::SFixed64,
    FieldType::Double,
    FieldType::String,
    FieldType::Bytes,
    FieldType::Fixed32,
    FieldType::SFixed32,
    FieldType::Float,
    // FieldType::Enum,
    FieldType::Embedded,
    // FieldType::Repeated,
    // FieldType::StartGroup,
    // FieldType::EndGroup,
];

pub struct SimpleParser<'a> {
    syntax: Syntax,
    fields_order: &'a [FieldType],
}

impl<'a> SimpleParser<'a> {
    pub fn new() -> SimpleParser<'a> {
        SimpleParser {
            syntax: Syntax::Proto3,
            fields_order: SimpleFieldsOrder,
        }
    }

    pub fn try_deserialize_specific_field(
        &self,
        into: &[u8],
        field_type: FieldType,
    ) -> Result<(Box<dyn FieldTrait>, u64)> {
        log::debug!("deserialization: try deserialize as {:}", field_type);
        let mut field: Box<dyn FieldTrait> = (field_type).into();
        (*field).deserialize(into).and_then(|x| Ok((field, x)))
    }

    pub fn try_deserialize_field(&self, into: &[u8]) -> Result<(Box<dyn FieldTrait>, u64)> {
        for i in self.fields_order.iter() {
            let mut field: Box<dyn FieldTrait> = (*i).into();
            log::debug!("deserialization: try deserialize as {:}", i);
            match (*field).deserialize(into) {
                Ok(i) => {
                    log::debug!("deserialization: deserialize as {:} successed", i);
                    return Ok((field, i));
                }
                Err(_) => continue,
            };
        }

        Err(Error::new("Failed to parse bytes", None))
    }
}

impl<'a> Parser for SimpleParser<'a> {
    fn deserialize(&self, into: &[u8]) -> Result<Message> {
        let mut fields = Vec::new();
        let mut index: u64 = 0;
        while index != into.len() as u64 {
            log::debug!(
                "deserialization: current_index - {:?}, data_len - {:?}",
                index,
                into.len()
            );
            match self.try_deserialize_field(&into[index as usize..]) {
                Ok((s, i)) => {
                    fields.push(s);
                    index += i;
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }
        Ok(Message::new("Generated".to_string(), Some(fields)))
    }
}

pub struct FullParser<'a> {
    syntax: Syntax,
    fields_order: &'a [FieldType],
}

impl<'a> FullParser<'a> {
    pub fn new() -> FullParser<'a> {
        FullParser {
            syntax: Syntax::Proto3,
            fields_order: SimpleFieldsOrder,
        }
    }

    pub fn try_deserialize_specific_field(
        &self,
        into: &[u8],
        field_type: FieldType,
    ) -> Result<(Box<dyn FieldTrait>, u64)> {
        log::debug!("deserialization: try deserialize as {:}", field_type);
        let mut field: Box<dyn FieldTrait> = (field_type).into();
        (*field).deserialize(into).and_then(|x| Ok((field, x)))
    }

    pub fn try_deserialize_field(&self, into: &[u8]) -> Result<(Box<dyn FieldTrait>, u64)> {
        for i in self.fields_order.iter() {
            let mut field: Box<dyn FieldTrait> = (*i).into();
            log::debug!("deserialization: try deserialize as {:}", i);
            match (*field).deserialize(into) {
                Ok(i) => {
                    log::debug!("deserialization: deserialize as {:} successed", i);
                    return Ok((field, i));
                }
                Err(_) => continue,
            };
        }

        Err(Error::new("Failed to parse bytes", None))
    }

    pub fn deserialize_fields(&self, into: &[u8]) -> Result<(Vec<Box<dyn FieldTrait>>, u64)> {
        let mut fields = Vec::new();
        let mut index: u64 = 0;
        while index != into.len() as u64 {
            log::debug!(
                "deserialization: current_index - {:?}, data_len - {:?}",
                index,
                into.len()
            );
            for filed_type in self.fields_order.iter() {
                match *filed_type {
                    FieldType::Embedded => {
                        match self.try_deserialize_specific_field(&into[index as usize..], *filed_type) {
                            Ok((mut s_em, i)) => {
                                log::error!("deserialization: deserialize as {:}(size: {:}) successed\n\n", filed_type, i);
                                let embedded =
                                    match self.deserialize_fields(&into[(index + i) as usize..]) {
                                        Ok((s, i)) => {
                                            index += i;
                                            s
                                        }
                                        Err(e) => {
                                            log::error!("{:}", e);
                                            continue;
                                        }
                                    };

                                match s_em.as_any().downcast_mut::<EmbeddedField>() {
                                    Some(b) => {
                                        b.0.data.fields = embedded;
                                    }
                                    None => panic!("&a isn't a B!"),
                                };
                                fields.push(s_em);
                                index += i;
                                break;
                            }
                            Err(e) => {
                                log::error!("{:}", e);
                                continue;
                            }
                        };
                    }
                    _ => {
                        match self.try_deserialize_specific_field(&into[index as usize..], *filed_type) {
                            Ok((s, i)) => {
                                log::error!("deserialization: deserialize as {:}(size: {:}) successed\n\n", filed_type, i);
                                fields.push(s);
                                index += i;
                                break;
                            }
                            Err(e) => {
                                log::error!("{:}", e);
                                continue;
                            }
                        };
                    }
                }
            }
            // return Err(Error::new("Failed to parse", None));
        }
        Ok((fields, index))
    }
}

impl<'a> Parser for FullParser<'a> {
    fn deserialize(&self, into: &[u8]) -> Result<Message> {
        let (x, s) = self.deserialize_fields(into)?;
        Ok(Message::new("Generated".to_string(), Some(x)))
    }
}