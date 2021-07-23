use core::convert::From;
use core::fmt;

use crate::parser::error::{Error, ErrorType, Result};
use crate::parser::utils::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VariantTypeRaw {
    /// Used for int32, int64, UInt32, UInt64, sint32, sint64, bool, enum
    Varint = 0,
    /// Used for fixed64, SFixed64, double
    Double = 1,
    /// Used for string, bytes, embedded messages, packed repeated fields
    Buffer = 2,
    /// Used for groups (deprecated)
    StartGroup = 3,
    /// Used for groups (deprecated)
    EndGroup = 4,
    /// Used for fixed32, SFixed32, float
    Float = 5,
}

impl fmt::Display for VariantTypeRaw {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}[{}]",
            match *self {
                VariantTypeRaw::Varint => "VariantTypeRaw::Varint",
                VariantTypeRaw::Double => "VariantTypeRaw::Double",
                VariantTypeRaw::Buffer => "VariantTypeRaw::Buffer",
                VariantTypeRaw::StartGroup => "VariantTypeRaw::StartGroup",
                VariantTypeRaw::EndGroup => "VariantTypeRaw::EndGroup",
                VariantTypeRaw::Float => "VariantTypeRaw::Float",
            },
            *self as u8
        )
    }
}

impl From<u8> for VariantTypeRaw {
    fn from(item: u8) -> Self {
        match item {
            0 => VariantTypeRaw::Varint,
            1 => VariantTypeRaw::Double,
            2 => VariantTypeRaw::Buffer,
            3 => VariantTypeRaw::StartGroup,
            4 => VariantTypeRaw::EndGroup,
            5 => VariantTypeRaw::Float,
            // Unreal sutuation
            _ => VariantTypeRaw::StartGroup,
        }
    }
}

/// Protobuf supported field types
#[derive(Debug, Clone, Copy, PartialEq)]
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
    /// Protobuf UInt32
    ///
    /// Uses variable-length encoding.
    UInt32,
    /// Protobuf UInt64
    ///
    /// Uses variable-length encoding.
    UInt64,
    /// Protobuf sint32
    ///
    /// Uses ZigZag variable-length encoding. Signed int value. These more efficiently
    /// encode negative numbers than regular int32s.
    SInt32,
    /// Protobuf sint64
    ///
    /// Uses ZigZag variable-length encoding. Signed int value. These more efficiently
    /// encode negative numbers than regular int32s.
    SInt64,
    /// Protobuf bool
    Bool,
    /// Protobuf fixed64
    ///
    /// Always eight bytes. More efficient than UInt64 if values are often greater than 2^56.
    Fixed64,
    /// Protobuf SFixed64
    ///
    /// Always eight bytes.
    SFixed64,
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
    /// Always four bytes. More efficient than UInt32 if values are often greater than 2^28.
    Fixed32,
    /// Protobuf SFixed32
    ///
    /// Always four bytes.
    SFixed32,
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
            | FieldType::UInt32
            | FieldType::UInt64
            | FieldType::SInt32
            | FieldType::SInt64
            | FieldType::Bool
            | FieldType::Enum => VariantTypeRaw::Varint,
            FieldType::Fixed64 | FieldType::SFixed64 | FieldType::Double => VariantTypeRaw::Double,
            FieldType::Embedded | FieldType::Repeated | FieldType::Bytes | FieldType::String => {
                VariantTypeRaw::Buffer
            }
            FieldType::StartGroup => VariantTypeRaw::StartGroup,
            FieldType::EndGroup => VariantTypeRaw::EndGroup,
            FieldType::Fixed32 | FieldType::SFixed32 | FieldType::Float => VariantTypeRaw::Float,
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
    fn serialize(&self) -> Vec<u8>;
    fn serialize_into(&self, into: &mut Vec<u8>);
    fn deserialize(&mut self, into: &[u8]) -> Result<u64>;
    //fn deserialize_into(into: &[u8]) -> Result<(Self, u64)>;
}

impl<T> Field<T>
where
    T: Clone,
{
    fn new(name: String, rule: FieldLabel, type_: FieldType, number: u64, data: T) -> Self {
        Field {
            name: name,
            rule: rule,
            type_: type_,
            number: number,
            data: data,
        }
    }
}

/// Filed with type Int32
#[derive(Debug, Clone, PartialEq)]
pub struct Int32Field(pub Field<i32>);

impl Int32Field {
    fn new(name: String, number: u64, data: i32) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Int32, number, data),
        }
    }
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

impl FieldTrait for Int32Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;
        if (value >> 0x32) != 0 {
            return Err(Error::new(
                "expected `Int32` found `U/Int64`",
                Some(ErrorType::IncorrectData),
            ));
        }

        self.0.data = value as i32;
        self.0.number = index;
        self.0.type_ = FieldType::Int32;

        Ok(readed + readed_x)
    }
}

/// Filed with type Int64
#[derive(Debug, Clone, PartialEq)]
pub struct Int64Field(pub Field<i64>);

impl Int64Field {
    fn new(name: String, number: u64, data: i64) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Int64, number, data),
        }
    }
}

impl Default for Int64Field {
    fn default() -> Self {
        Int64Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Int64,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for Int64Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;

        self.0.data = value as i64;
        self.0.number = index;
        self.0.type_ = FieldType::Int32;

        Ok(readed + readed_x)
    }
}

/// Filed with type UInt32
#[derive(Debug, Clone, PartialEq)]
pub struct UInt32Field(pub Field<u32>);

impl UInt32Field {
    fn new(name: String, number: u64, data: u32) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::UInt32, number, data),
        }
    }
}

impl Default for UInt32Field {
    fn default() -> Self {
        UInt32Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::UInt32,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for UInt32Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;
        if (value >> 0x32) != 0 {
            return Err(Error::new(
                "expected `UInt32` found `U/Int64`",
                Some(ErrorType::IncorrectData),
            ));
        }

        self.0.data = value as u32;
        self.0.number = index;
        self.0.type_ = FieldType::UInt32;

        Ok(readed + readed_x)
    }
}

/// Filed with type UInt64
#[derive(Debug, Clone, PartialEq)]
pub struct UInt64Field(pub Field<i64>);

impl UInt64Field {
    fn new(name: String, number: u64, data: i64) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::UInt64, number, data),
        }
    }
}

impl Default for UInt64Field {
    fn default() -> Self {
        UInt64Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::UInt64,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for UInt64Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;

        self.0.data = value as i64;
        self.0.number = index;
        self.0.type_ = FieldType::Int32;

        Ok(readed + readed_x)
    }
}

/// Filed with type SInt32
#[derive(Debug, Clone, PartialEq)]
pub struct SInt32Field(pub Field<i32>);

impl SInt32Field {
    fn new(name: String, number: u64, data: i32) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::UInt32, number, data),
        }
    }
}

impl Default for SInt32Field {
    fn default() -> Self {
        SInt32Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::SInt32,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for SInt32Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(encode_zigzag_s32(self.0.data), into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;
        if (value >> 0x32) != 0 {
            return Err(Error::new(
                "expected `SUInt32` found `U/Int64`",
                Some(ErrorType::IncorrectData),
            ));
        }

        self.0.data = decode_zigzag_s32(value);
        self.0.number = index;
        self.0.type_ = FieldType::UInt32;

        Ok(readed + readed_x)
    }
}

/// Filed with type UInt64
#[derive(Debug, Clone, PartialEq)]
pub struct SInt64Field(pub Field<i64>);

impl SInt64Field {
    fn new(name: String, number: u64, data: i64) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::SInt64, number, data),
        }
    }
}

impl Default for SInt64Field {
    fn default() -> Self {
        SInt64Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::SInt64,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for SInt64Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(encode_zigzag_s64(self.0.data), into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;

        self.0.data = decode_zigzag_s64(value);
        self.0.number = index;
        self.0.type_ = FieldType::Int32;

        Ok(readed + readed_x)
    }
}

/// Filed with type Bool
#[derive(Debug, Clone, PartialEq)]
pub struct BoolField(pub Field<bool>);

impl BoolField {
    fn new(name: String, number: u64, data: bool) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Bool, number, data),
        }
    }
}

impl Default for BoolField {
    fn default() -> Self {
        BoolField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Bool,
                number: 0,
                data: false,
            },
        }
    }
}

impl FieldTrait for BoolField {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Varint as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Varint,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;
        if (value >> 0x1) != 0 {
            return Err(Error::new(
                "expected `Boolean` found `U/Int32/64`",
                Some(ErrorType::IncorrectData),
            ));
        }
        self.0.data = value != 0;
        self.0.number = index;
        self.0.type_ = FieldType::Bool;

        Ok(readed + readed_x)
    }
}

/// Filed with type Fixed32
#[derive(Debug, Clone, PartialEq)]
pub struct Fixed32Field(pub Field<i32>);

impl Fixed32Field {
    fn new(name: String, number: u64, data: i32) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Fixed32, number, data),
        }
    }
}

impl Default for Fixed32Field {
    fn default() -> Self {
        Fixed32Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Fixed32,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for Fixed32Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        into.extend_from_slice(&self.0.data.to_le_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Float as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Float,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        if into[readed as usize..].len() < 4 {
            return Err(Error::new(
                &format!("expected 4 bytes found `{}`", into[readed as usize..].len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..4];
        let value = i32::from_le_bytes([ptr[0], ptr[1], ptr[2], ptr[3]]);
        let readed_x = 0x04;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Fixed32;

        Ok(readed + readed_x)
    }
}

/// Filed with type SFixed32
#[derive(Debug, Clone, PartialEq)]
pub struct SFixed32Field(pub Field<u32>);

impl SFixed32Field {
    fn new(name: String, number: u64, data: u32) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Fixed32, number, data),
        }
    }
}

impl Default for SFixed32Field {
    fn default() -> Self {
        SFixed32Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::SFixed32,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for SFixed32Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        into.extend_from_slice(&self.0.data.to_le_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Float as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Float,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        if into[readed as usize..].len() < 8 {
            return Err(Error::new(
                &format!("expected 8 bytes found `{}`", into[readed as usize..].len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..4];
        let value = u32::from_le_bytes([ptr[0], ptr[1], ptr[2], ptr[3]]);
        let readed_x = 0x04;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::SFixed32;

        Ok(readed + readed_x)
    }
}

/// Filed with type Float
#[derive(Debug, Clone, PartialEq)]
pub struct FloatField(pub Field<f32>);

impl FloatField {
    fn new(name: String, number: u64, data: f32) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Float, number, data),
        }
    }
}

impl Default for FloatField {
    fn default() -> Self {
        FloatField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Float,
                number: 0,
                data: 0.0,
            },
        }
    }
}

impl FieldTrait for FloatField {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        into.extend_from_slice(&self.0.data.to_le_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Float as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Float,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        if into[readed as usize..].len() < 4 {
            return Err(Error::new(
                &format!("expected 4 bytes found `{}`", into[readed as usize..].len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..4];
        let value = f32::from_le_bytes([ptr[0], ptr[1], ptr[2], ptr[3]]);
        let readed_x = 0x04;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Float;

        Ok(readed + readed_x)
    }
}

/// Filed with type Fixed64
#[derive(Debug, Clone, PartialEq)]
pub struct Fixed64Field(pub Field<i64>);

impl Fixed64Field {
    fn new(name: String, number: u64, data: i64) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Fixed64, number, data),
        }
    }
}

impl Default for Fixed64Field {
    fn default() -> Self {
        Fixed64Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Fixed64,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for Fixed64Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        into.extend_from_slice(&self.0.data.to_le_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Double as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Double,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        if into[readed as usize..].len() < 8 {
            return Err(Error::new(
                &format!("expected 8 bytes found `{}`", into[readed as usize..].len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..8];
        let value = i64::from_le_bytes([
            ptr[0], ptr[1], ptr[2], ptr[3], ptr[4], ptr[5], ptr[6], ptr[7],
        ]);
        let readed_x = 0x08;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Fixed64;

        Ok(readed + readed_x)
    }
}

/// Filed with type SFixed64
#[derive(Debug, Clone, PartialEq)]
pub struct SFixed64Field(pub Field<u64>);

impl SFixed64Field {
    fn new(name: String, number: u64, data: u64) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Fixed64, number, data),
        }
    }
}

impl Default for SFixed64Field {
    fn default() -> Self {
        SFixed64Field {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::SFixed64,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for SFixed64Field {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        into.extend_from_slice(&self.0.data.to_le_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Double as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Double,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        if into[readed as usize..].len() < 8 {
            return Err(Error::new(
                &format!("expected 8 bytes found `{}`", into[readed as usize..].len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..8];
        let value = u64::from_le_bytes([
            ptr[0], ptr[1], ptr[2], ptr[3], ptr[4], ptr[5], ptr[6], ptr[7],
        ]);
        let readed_x = 0x08;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::SFixed64;

        Ok(readed + readed_x)
    }
}

/// Filed with type SFixed64
#[derive(Debug, Clone, PartialEq)]
pub struct DoubleField(pub Field<f64>);

impl DoubleField {
    fn new(name: String, number: u64, data: f64) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Double, number, data),
        }
    }
}

impl Default for DoubleField {
    fn default() -> Self {
        DoubleField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Double,
                number: 0,
                data: 0.0,
            },
        }
    }
}

impl FieldTrait for DoubleField {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        into.extend_from_slice(&self.0.data.to_le_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Varint`
        if type_int != VariantTypeRaw::Double as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Double,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        if into[readed as usize..].len() < 8 {
            return Err(Error::new(
                &format!("expected 8 bytes found `{}`", into[readed as usize..].len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..8];
        let value = f64::from_le_bytes([
            ptr[0], ptr[1], ptr[2], ptr[3], ptr[4], ptr[5], ptr[6], ptr[7],
        ]);
        let readed_x = 0x08;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Double;

        Ok(readed + readed_x)
    }
}

/// Filed with type String
#[derive(Debug, Clone, PartialEq)]
pub struct StringField(pub Field<String>);

impl StringField {
    fn new(name: String, number: u64, data: String) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::String, number, data),
        }
    }
}

impl Default for StringField {
    fn default() -> Self {
        StringField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::String,
                number: 0,
                data: "".to_string(),
            },
        }
    }
}

impl FieldTrait for StringField {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data.len() as u64, into);
        into.extend_from_slice(&self.0.data.as_bytes());
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Buffer`
        if type_int != VariantTypeRaw::Buffer as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Buffer,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (size, readed_1) = deserialize_varint(&into[readed as usize..])?;
        if into[(readed + readed_1) as usize..].len() < size as usize {
            return Err(Error::new(
                &format!(
                    "expected {} bytes found `{}`",
                    size,
                    into[readed as usize..].len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }
        let value = String::from_utf8(
            into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize].to_vec(),
        )
        .or(Err(Error::new(
            &format!("Failed to create String from bytes"),
            Some(ErrorType::IncorrectData),
        )))?;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::String;

        Ok(readed + readed + readed_1 + size)
    }
}

/// Filed with type Bytes
#[derive(Debug, Clone, PartialEq)]
pub struct BytesField(pub Field<Vec<u8>>);

impl BytesField {
    fn new(name: String, number: u64, data: &[u8]) -> Self {
        Self {
            0: Field::new(
                name,
                FieldLabel::Optional,
                FieldType::Bytes,
                number,
                data.to_vec(),
            ),
        }
    }
}

impl Default for BytesField {
    fn default() -> Self {
        BytesField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Bytes,
                number: 0,
                data: Vec::new(),
            },
        }
    }
}

impl FieldTrait for BytesField {
    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(self.0.data.len() as u64, into);
        into.extend_from_slice(&self.0.data);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Buffer`
        if type_int != VariantTypeRaw::Buffer as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Buffer,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (size, readed_1) = deserialize_varint(&into[readed as usize..])?;
        if into[(readed + readed_1) as usize..].len() < size as usize {
            return Err(Error::new(
                &format!(
                    "expected {} bytes found `{}`",
                    size,
                    into[readed as usize..].len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }
        let value =
            into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize].to_vec();
        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Bytes;

        Ok(readed + readed + readed_1 + size)
    }
}

/// Filed with type Embedded
#[derive(Debug, Clone, PartialEq)]
pub struct EmbeddedField<T: FieldTrait + Clone + Default>(pub Field<T>);

impl<T> EmbeddedField<T>
where
    T: FieldTrait + Clone + Default,
{
    fn new(name: String, number: u64, data: T) -> Self {
        Self {
            0: Field::new(name, FieldLabel::Optional, FieldType::Bytes, number, data),
        }
    }
}

impl<T> Default for EmbeddedField<T>
where
    T: FieldTrait + Clone + Default,
{
    fn default() -> Self {
        EmbeddedField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Bytes,
                number: 0,
                data: T::default(),
            },
        }
    }
}

impl<T> FieldTrait for EmbeddedField<T>
where
    T: FieldTrait + Default + Clone,
{
    fn serialize_into(&self, into: &mut Vec<u8>) {
        let embedded = self.0.data.serialize();
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        serialize_varint_into(embedded.len() as u64, into);
        into.extend(&embedded);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::Buffer`
        if type_int != VariantTypeRaw::Buffer as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::Buffer,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        let (size, readed_1) = deserialize_varint(&into[readed as usize..])?;
        if into[(readed + readed_1) as usize..].len() < size as usize {
            return Err(Error::new(
                &format!(
                    "expected {} bytes found `{}`",
                    size,
                    into[readed as usize..].len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }
        let mut value = T::default();
        value.deserialize(
            &into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize],
        )?;
        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Embedded;

        Ok(readed + readed + readed_1 + size)
    }
}
