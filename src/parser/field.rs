use core::convert::From;

use crate::parser::error::{Error, ErrorType, Result};
use crate::parser::utils::*;

pub enum VariantTypeRaw {
    /// Used for int32, int64, uint32, uint64, sint32, sint64, bool, enum
    Varint = 0,
    /// Used for fixed64, sfixed64, double
    Double = 1,
    /// Used for string, bytes, embedded messages, packed repeated fields
    Buffer = 2,
    /// Used for groups (deprecated)
    StartGroup = 3,
    /// Used for groups (deprecated)
    EndGroup = 4,
    /// Used for fixed32, sfixed32, float
    Float = 5,
}

/// Protobuf supported field types
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    /// Protobuf int32
    ///
    /// Uses variable-length encoding. Inefficient for encoding negative numbers – if
    /// your field is likely to have negative values, use sint32 instead.
    Int32,
    /// Protobuf int64
    ///
    /// Uses variable-length encoding. Inefficient for encoding negative numbers – if
    /// your field is likely to have negative values, use sint64 instead.
    Int64,
    /// Protobuf uint32
    ///
    /// Uses variable-length encoding.
    Uint32,
    /// Protobuf uint64
    ///
    /// Uses variable-length encoding.
    Uint64,
    /// Protobuf sint32
    ///
    /// Uses ZigZag variable-length encoding. Signed int value. These more efficiently
    /// encode negative numbers than regular int32s.
    Sint32,
    /// Protobuf sint64
    ///
    /// Uses ZigZag variable-length encoding. Signed int value. These more efficiently
    /// encode negative numbers than regular int32s.
    Sint64,
    /// Protobuf bool
    Bool,
    /// Protobuf fixed64
    ///
    /// Always eight bytes. More efficient than uint64 if values are often greater than 2^56.
    Fixed64,
    /// Protobuf sfixed64
    ///
    /// Always eight bytes.
    Sfixed64,
    /// Protobuf double
    Double,
    /// Protobuf string
    ///
    /// A string must always contain UTF-8 encoded or 7-bit ASCII text.
    String,
    /// Protobuf bytes
    ///
    /// May contain any arbitrary sequence of bytes.
    Bytes,
    /// Protobuf fixed32
    ///
    /// Always four bytes. More efficient than uint32 if values are often greater than 2^28.
    Fixed32,
    /// Protobuf sfixed32
    ///
    /// Always four bytes.
    Sfixed32,
    /// Protobuf float
    Float,
    /// Protobuf enum
    ///
    ///
    Enum,
    /// Protobuf embedded messages
    ///
    ///
    Embedded,
    /// Protobuf packed repeated messages
    ///
    ///
    Repeated,
    StartGroup,
    EndGroup,
}

impl From<FieldType> for VariantTypeRaw {
    fn from(item: FieldType) -> Self {
        match item {
            FieldType::Int32
            | FieldType::Int64
            | FieldType::Uint32
            | FieldType::Uint64
            | FieldType::Sint32
            | FieldType::Sint64
            | FieldType::Bool
            | FieldType::Enum => VariantTypeRaw::Varint,
            FieldType::Fixed64 | FieldType::Sfixed64 | FieldType::Double => VariantTypeRaw::Double,
            FieldType::Embedded | FieldType::Repeated | FieldType::Bytes | FieldType::String => {
                VariantTypeRaw::Buffer
            }
            FieldType::StartGroup => VariantTypeRaw::StartGroup,
            FieldType::EndGroup => VariantTypeRaw::EndGroup,
            FieldType::Fixed32 | FieldType::Sfixed32 | FieldType::Float => VariantTypeRaw::Float,
        }
    }
}

/// A field rule
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FieldLabel {
    /// A well-formed message can have zero or one of this field (but not more than one).
    Optional,
    /// This field can be repeated any number of times (including zero) in a well-formed message.
    /// The order of the repeated values will be preserved.
    Repeated,
    /// A well-formed message must have exactly one of this field.
    Required,
}

/// A Protobuf Field
#[derive(Debug, Clone, PartialEq)]
pub struct Field<T>
where
    T: Clone,
{
    /// Field name
    pub name: String,
    /// Field `Rule`
    pub rule: FieldLabel,
    /// Field type
    pub type_: FieldType,
    /// Tag number
    pub number: u64,
    /// Data
    pub data: T,
}

pub trait FieldTrait {
    //TODO: onwers addd
    fn serialize(self) -> Vec<u8>;
    fn serialize_into(self, into: &mut Vec<u8>);

    fn deserialize(&mut self, into: &[u8]) -> Result<u64>;
    //fn deserialize_into(into: &[u8]) -> Result<(Self, u64)>;

    //fn new(name: String, rule: FieldLabel, type_: FieldType, number: u64, data: T) -> Self;
}

/// Filed with type Int32
pub struct Int32Field(pub Field<i32>);

impl Int32Field {
    fn new(name: String, rule: FieldLabel, type_: FieldType, number: u64, data: i32) -> Self {
        Self {
            0: Field {
                name: name,
                rule: rule,
                type_: type_,
                number: number,
                data: data,
            },
        }
    }
}

impl FieldTrait for Int32Field {
    fn serialize_into(self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, mut readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        if type_int != VariantTypeRaw::Varint as u8 {
            //TODO: Add norm print
            return Err(Error::new(
                &format!("expected `VariantTypeRaw::Varint` found `{}`", type_int),
                Some(ErrorType::IncorrectType),
            ));
        }
        //TODO: Add check of value's size
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;
        self.0.data = value as i32;
        self.0.number = index;
        self.0.type_ = FieldType::Int32;

        Ok(readed + readed_x)
    }

    // fn deserialize_into(into: &[u8]) -> Result<(Self, u64)> {
    //     let mut s = Int32Field {
    //         0: Field {
    //             name: "".to_string(),
    //             rule: FieldLabel::Optional,
    //             type_: FieldType::Int32,
    //             number: 0,
    //             data: 0,
    //         },
    //     };

    //     s.deserialize(into).and_then(|readed| Ok((s, readed)))
    // }
}

impl Default for Int32Field {
    fn default() -> Self {
        Int32Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Int32,
                number: 0,
                data: 0,
            },
        }
    }
}
