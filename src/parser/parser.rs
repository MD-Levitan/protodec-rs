use std::collections::BTreeMap;

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
    FieldType::Fixed32,
    FieldType::SFixed32,
    FieldType::Float,
    FieldType::String,
    FieldType::Embedded,
    FieldType::Bytes,
    // FieldType::Enum,
    // FieldType::Repeated,
    FieldType::StartGroup,
    // FieldType::EndGroup,
];

pub struct SimpleParser<'a> {
    syntax: Syntax,
    fields_order: &'a [FieldType],
}

/// Trying to decode/deserialize bytes into field
pub(crate) fn try_deserialize_specific_field(
    into: &[u8],
    field_type: FieldType,
) -> Result<(Box<dyn FieldTrait>, u64)> {
    log::debug!("Deserialization: try deserialize as {:}", field_type);
    let mut field: Box<dyn FieldTrait> = (field_type).into();
    (*field).deserialize(into).and_then(|x| Ok((field, x)))
}

pub(crate) fn try_deserialize_field<'a, I: Iterator<Item = &'a FieldType>>(
    into: &[u8],
    fields_type: I,
) -> Result<(Box<dyn FieldTrait>, u64)> {
    for type_i in fields_type {
        let mut field: Box<dyn FieldTrait> = (*type_i).into();
        log::debug!("Deserialization: try deserialize as {:}", type_i);
        match (*field).deserialize(into) {
            Ok(i) => {
                log::debug!("Deserialization: deserialize as {:} successed", i);
                return Ok((field, i));
            }
            Err(_) => continue,
        };
    }

    Err(Error::new("Failed to parse bytes", None))
}

impl<'a> SimpleParser<'a> {
    pub fn new() -> SimpleParser<'a> {
        SimpleParser {
            syntax: Syntax::Proto3,
            fields_order: SimpleFieldsOrder,
        }
    }

    pub fn try_deserialize_field(&self, into: &[u8]) -> Result<(Box<dyn FieldTrait>, u64)> {
        try_deserialize_field(into, self.fields_order.iter())
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

    pub fn deserialize_fields(&self, into: &[u8]) -> Result<(Vec<Box<dyn FieldTrait>>, u64)> {
        let mut fields = Vec::new();
        let mut index: u64 = 0;
        while index != into.len() as u64 {
            log::debug!(
                "Deserialization Loop: current_index - {:?}, data_len - {:?}",
                index,
                into.len()
            );
            let mut found = false;
            for field_type in self.fields_order.iter() {
                match *field_type {
                    FieldType::Embedded => {
                        match try_deserialize_specific_field(&into[index as usize..], *field_type) {
                            Ok((mut s_em, i)) => {
                                log::info!("Deserialization: deserialize as {:} (size: {:}) successed {:}\n\n", field_type, i, s_em.repr());
                                match s_em.as_any().downcast_mut::<EmbeddedField>() {
                                    Some(b) => match &b.raw {
                                        Some(data) => {
                                            let embedded = match self.deserialize_fields(&data) {
                                                Ok((s, _)) => s,
                                                Err(e) => {
                                                    log::info!("{:}", e);
                                                    continue;
                                                }
                                            };
                                            b.field.data.fields = embedded;
                                        }
                                        None => {
                                            log::info!("{:}", "Failed to create Embedded 1");
                                            continue;
                                        }
                                    },
                                    None => {
                                        log::info!(
                                            "{:}  {:?}",
                                            "Failed to create Embedded",
                                            s_em.repr()
                                        );
                                        continue;
                                    }
                                };
                                fields.push(s_em);
                                index += i;
                                found = true;
                                break;
                            }
                            Err(e) => {
                                log::info!("{:}", e);
                                continue;
                            }
                        };
                    }
                    _ => {
                        match try_deserialize_specific_field(&into[index as usize..], *field_type) {
                            Ok((s, i)) => {
                                log::info!("deserialization: deserialize as {:}(size: {:}) successed{:}\n\n", field_type, i, s.repr());
                                fields.push(s);
                                index += i;
                                found = true;
                                break;
                            }
                            Err(e) => {
                                log::info!("{:}", e);
                                continue;
                            }
                        };
                    }
                }
            }
            if found == false {
                return Err(Error::new("Failed to find suitable field", None));
            }
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

pub struct PartialParser<'a> {
    syntax: Syntax,
    fields_order: &'a [FieldType],
}

impl<'a> PartialParser<'a> {
    pub fn new() -> PartialParser<'a> {
        Self {
            syntax: Syntax::Proto3,
            fields_order: SimpleFieldsOrder,
        }
    }

    pub fn deserialize_fields(&self, into: &[u8]) -> Result<(Vec<Box<dyn FieldTrait>>, u64)> {
        let mut fields = Vec::new();
        let mut index: u64 = 0;
        while index != into.len() as u64 {
            log::debug!(
                "Deserialization Loop: current_index - {:?}, data_len - {:?}",
                index,
                into.len()
            );
            let mut found = false;
            for field_type in self.fields_order.iter() {
                match *field_type {
                    FieldType::Embedded => {
                        match try_deserialize_specific_field(&into[index as usize..], *field_type) {
                            Ok((mut s_em, i)) => {
                                log::info!("Deserialization: deserialize as {:} (size: {:}) successed {:}\n\n", field_type, i, s_em.repr());
                                match s_em.as_any().downcast_mut::<EmbeddedField>() {
                                    Some(b) => match &b.raw {
                                        Some(data) => {
                                            let embedded = match self.deserialize_fields(&data) {
                                                Ok((s, _)) => s,
                                                Err(e) => {
                                                    log::info!("{:}", e);
                                                    continue;
                                                }
                                            };
                                            b.field.data.fields = embedded;
                                        }
                                        None => {
                                            log::info!("{:}", "Failed to create Embedded 1");
                                            continue;
                                        }
                                    },
                                    None => {
                                        log::info!(
                                            "{:}  {:?}",
                                            "Failed to create Embedded",
                                            s_em.repr()
                                        );
                                        continue;
                                    }
                                };
                                fields.push(s_em);
                                index += i;
                                found = true;
                                break;
                            }
                            Err(e) => {
                                log::info!("{:}", e);
                                continue;
                            }
                        };
                    }
                    _ => {
                        match try_deserialize_specific_field(&into[index as usize..], *field_type) {
                            Ok((s, i)) => {
                                log::info!("deserialization: deserialize as {:}(size: {:}) successed{:}\n\n", field_type, i, s.repr());
                                fields.push(s);
                                index += i;
                                found = true;
                                break;
                            }
                            Err(e) => {
                                log::info!("{:}", e);
                                continue;
                            }
                        };
                    }
                }
            }
            if found == false {
                return Ok((fields, index));
            }
        }
        Ok((fields, index))
    }

    pub fn deserialize_map(&self, into: &[u8]) -> BTreeMap<(usize, usize), Message> {
        let mut hashmap = BTreeMap::new();

        for start_bytes in 0..into.len() as usize {
            if let Ok((message, end_bytes)) = self.deserialize_fields(&into[start_bytes..]) {
                if !message.is_empty() {
                    hashmap.insert(
                        (start_bytes, start_bytes + end_bytes as usize),
                        Message::new("Generated".to_string(), Some(message)),
                    );
                }
            }
        }

        hashmap
    }
}

#[cfg(test)]
mod test {
    use crate::proto::message;

    use super::*;

    #[test]
    fn test_deserialize_partial_parser() {
        let buffer = [
            0x08, 0xbc, 0xec, 0x8e, 0x90, 0x06, 0x12, 0x00, 0x1a, 0x10, 0x66, 0x33, 0x64, 0x31,
            0x36, 0x33, 0x35, 0x32, 0x39, 0x33, 0x33, 0x38, 0x32, 0x66, 0x34, 0x62, 0x22, 0x11,
            0x63, 0x6f, 0x6d, 0x2e, 0x74, 0x7a, 0x7a, 0x2e, 0x73, 0x69, 0x67, 0x6e, 0x6d, 0x61,
        ];
        let deserializer = PartialParser::new();
        let map = deserializer.deserialize_map(&buffer);

        for (bounds, value) in map.iter() {
            println!("data[{:x}:{:x}] - {:?}", bounds.0, bounds.1, value);
        }
    }
}
