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

impl<'a> SimpleParser<'a>
{
    pub fn new() -> SimpleParser<'a>
    {
        SimpleParser
        {
            syntax: Syntax::Proto3,
            fields_order: SimpleFieldsOrder,
        }
    }

    pub fn try_deserialize_specific_field(&self, into: &[u8], field_type: FieldType) -> Result<(Box<dyn FieldTrait>, u64)>
    {
        let mut field:Box<dyn FieldTrait> = (field_type).into();
        (*field).deserialize(into).and_then(|x| Ok((field, x)))
    }

}

impl<'a> Parser for SimpleParser<'a> {
    
    fn deserialize(&self, into: &[u8]) -> Result<Message> {
        let mut index: u64 = 0;
        while index != into.len() as u64
        {

        }
        Ok(Message::new("Generated".to_string(), None))
    }
}
