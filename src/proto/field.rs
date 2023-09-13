#![allow(dead_code)]

use core::any::Any;
use core::convert::From;
use core::fmt;
use core::ops::Add;

use crate::proto::error::{Error, ErrorType, Result};
use crate::proto::utils::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VariantTypeRaw {
    /// Used for int32, int64, UInt32, UInt64, sint32, sint64, bool, enum
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
    /// Undefined Type
    Undefined = -1,
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
                VariantTypeRaw::Undefined => "VariantTypeRaw::Undefined",
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
            _ => VariantTypeRaw::Undefined,
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

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}[{}]",
            match *self {
                FieldType::Int32 => "FieldType::Int32",
                FieldType::Int64 => "FieldType::Int64",
                FieldType::UInt32 => "FieldType::UInt32",
                FieldType::UInt64 => "FieldType::UInt64",
                FieldType::SInt32 => "FieldType::SInt32",
                FieldType::SInt64 => "FieldType::SInt64",
                FieldType::Bool => "FieldType::Bool",
                FieldType::Fixed64 => "FieldType::Fixed64",
                FieldType::SFixed64 => "FieldType::SFixed64",
                FieldType::Double => "FieldType::Double",
                FieldType::String => "FieldType::String",
                FieldType::Bytes => "FieldType::Bytes",
                FieldType::Fixed32 => "FieldType::Fixed32",
                FieldType::SFixed32 => "FieldType::SFixed32",
                FieldType::Float => "FieldType::Float",
                FieldType::Enum => "FieldType::Enum",
                FieldType::Embedded => "FieldType::Embedded",
                FieldType::Repeated => "FieldType::Repeated",
                FieldType::StartGroup => "FieldType::StartGroup",
                FieldType::EndGroup => "FieldType::EndGroup",
            },
            *self as u8
        )
    }
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

impl From<FieldType> for Box<dyn FieldTrait> {
    fn from(item: FieldType) -> Self {
        match item {
            FieldType::Int32 => Box::new(Int32Field::default()),
            FieldType::Int64 => Box::new(Int64Field::default()),
            FieldType::UInt32 => Box::new(UInt32Field::default()),
            FieldType::UInt64 => Box::new(UInt64Field::default()),
            FieldType::SInt32 => Box::new(SInt32Field::default()),
            FieldType::SInt64 => Box::new(SInt64Field::default()),
            FieldType::Bool => Box::new(BoolField::default()),
            FieldType::Enum => Box::new(Field::default()), //Box::new(EnumField::default()),
            FieldType::Fixed64 => Box::new(Fixed64Field::default()),
            FieldType::SFixed64 => Box::new(SFixed64Field::default()),
            FieldType::Double => Box::new(DoubleField::default()),
            FieldType::Embedded => Box::new(EmbeddedField::default()),
            FieldType::Repeated => Box::new(Field::default()), //Box::new(RepeatedField::default()),
            FieldType::Bytes => Box::new(BytesField::default()),
            FieldType::String => Box::new(StringField::default()),
            FieldType::StartGroup => Box::new(StartGroupField::default()),
            FieldType::EndGroup => Box::new(Field::default()), //Box::new(EndGroupField::default()),
            FieldType::Fixed32 => Box::new(Fixed32Field::default()),
            FieldType::SFixed32 => Box::new(Fixed64Field::default()),
            FieldType::Float => Box::new(FloatField::default()),
        }
    }
}

impl FieldType {
    fn to_str(&self) -> &str {
        match *self {
            FieldType::Int32 => "int32",
            FieldType::Int64 => "int64",
            FieldType::UInt32 => "uint32",
            FieldType::UInt64 => "uint64",
            FieldType::SInt32 => "sint32",
            FieldType::SInt64 => "sint64",
            FieldType::Bool => "bool",
            FieldType::Fixed64 => "fixed64",
            FieldType::SFixed64 => "sfixed64",
            FieldType::Double => "double",
            FieldType::String => "string",
            FieldType::Bytes => "bytes",
            FieldType::Fixed32 => "fixed32",
            FieldType::SFixed32 => "sfixed32",
            FieldType::Float => "float",
            FieldType::Enum => "enum",
            FieldType::Embedded => "embedded",
            FieldType::Repeated => "repeated",
            FieldType::StartGroup => "startgroup",
            FieldType::EndGroup => "endgroup",
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

impl fmt::Display for FieldLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                FieldLabel::Optional => "optional",
                FieldLabel::Repeated => "repeated",
                FieldLabel::Required => "required",
            }
        )
    }
}

/// A Protobuf Field
#[derive(Debug, Clone, PartialEq)]
pub struct Field<T> {
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

impl Default for Field<Vec<u8>> {
    fn default() -> Self {
        Field {
            name: "".to_string(),
            rule: FieldLabel::Optional,
            type_: FieldType::Bytes,
            number: 0,
            data: Vec::new(),
        }
    }
}

pub trait FieldTrait {
    fn serialize(&self) -> Vec<u8>;
    fn serialize_into(&self, into: &mut Vec<u8>);
    fn deserialize(&mut self, into: &[u8]) -> Result<u64>;
    fn as_any(&mut self) -> &mut dyn Any;
    fn repr(&self) -> String;
    fn to_str(&self, name: &str) -> String;
}

impl<T> Field<T> {
    fn new(name: String, rule: FieldLabel, type_: FieldType, number: u64, data: T) -> Self {
        Field {
            name: name,
            rule: rule,
            type_: type_,
            number: number,
            data: data,
        }
    }

    fn to_str(&self, data_repr: &str, name: &str) -> String {
        format!(
            "{rule} {type} {name} = {number};        // Example: {data}",
            number = self.number,
            rule = self.rule,
            type = self.type_.to_str(),
            data = data_repr.clone(),
            name = name
        )
    }

    fn repr(&self, data_repr: &str) -> String {
        format!(
            "{:#x} {} <{} == {}> = {}",
            self.number,
            self.rule,
            self.type_,
            VariantTypeRaw::from(self.type_),
            data_repr.clone()
        )
    }
}

impl FieldTrait for Field<Vec<u8>> {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        let data_repr = self.data.iter().fold(String::new(), |data_repr, x| {
            data_repr.add(&format!(" {:02X}", x))
        });
        self.repr(&data_repr)
    }

    fn to_str(&self, name: &str) -> String {
        let data_repr = self.data.iter().fold(String::new(), |data_repr, x| {
            data_repr.add(&format!(" {:02X}", x))
        });
        self.to_str(&data_repr, name)
    }

    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.number, VariantTypeRaw::from(self.type_) as u8),
            into,
        );
        serialize_varint_into(self.data.len() as u64, into);
        into.extend_from_slice(&self.data);
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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
            ));
        }

        let (size, readed_1) = deserialize_varint(&into[readed as usize..])?;
        if (readed + readed_1 + size) as usize > into.len() {
            return Err(Error::new(
                &format!(
                    "expected {} bytes, found `{}`",
                    (readed + readed_1 + size),
                    into.len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }
        let value =
            into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize].to_vec();
        self.data = value;
        self.number = index;
        self.type_ = FieldType::Bytes;

        Ok(readed + readed + readed_1 + size)
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
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
pub struct UInt64Field(pub Field<u64>);

impl UInt64Field {
    fn new(name: String, number: u64, data: u64) -> Self {
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
            ));
        }

        let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;

        self.0.data = value as u64;
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if (readed + 4) as usize > into.len() {
            return Err(Error::new(
                &format!("expected {} bytes, found `{}`", (readed + 4), into.len()),
                Some(ErrorType::IncorrectData),
            ));
        }

        let ptr = &into[readed as usize..(readed + 4) as usize];
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if (readed + 4) as usize > into.len() {
            return Err(Error::new(
                &format!("expected {} bytes, found `{}`", (readed + 4), into.len()),
                Some(ErrorType::IncorrectData),
            ));
        }
        let ptr = &into[readed as usize..(readed + 4) as usize];
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if (readed + 4) as usize > into.len() {
            return Err(Error::new(
                &format!("expected {} bytes, found `{}`", (readed + 4), into.len()),
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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
        if (readed + 8) as usize > into.len() {
            return Err(Error::new(
                &format!("expected {} bytes, found `{}`", (readed + 8), into.len()),
                Some(ErrorType::IncorrectData),
            ));
        }

        let ptr = &into[readed as usize..(readed + 8) as usize];
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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

        if (readed + 8) as usize > into.len() {
            return Err(Error::new(
                &format!("expected {} bytes, found `{}`", (readed + 8), into.len()),
                Some(ErrorType::IncorrectData),
            ));
        }

        let ptr = &into[readed as usize..(readed + 8) as usize];
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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
        if (readed + 8) as usize > into.len() {
            return Err(Error::new(
                &format!("expected {} bytes, found `{}`", (readed + 8), into.len()),
                Some(ErrorType::IncorrectData),
            ));
        }

        let ptr = &into[readed as usize..(readed + 8) as usize];
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        let data_repr = self
            .0
            .data
            .as_bytes()
            .iter()
            .fold(String::new(), |data_repr, x| {
                data_repr.add(&format!(" {:02X} ", x))
            });
        self.0.repr(&format!("{:} ({:})", &self.0.data, &data_repr))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

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
        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
            ));
        }

        let (size, readed_1) = deserialize_varint(&into[readed as usize..])?;
        if (readed + readed_1 + size) as usize > into.len() {
            return Err(Error::new(
                &format!(
                    "expected {} bytes, found `{}`",
                    (readed + readed_1 + size),
                    into.len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }

        let str_vec =
            into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize].to_vec();

        if let Some(_) = str_vec.iter().find(|&&x| x < 0x20 || x > 0x7F) {
            return Err(Error::new(
                &format!("Failed to create String from bytes(non ASCII)"),
                Some(ErrorType::IncorrectData),
            ))?;
        }

        let value = String::from_utf8(str_vec).or(Err(Error::new(
            &format!("Failed to create String from bytes"),
            Some(ErrorType::IncorrectData),
        )))?;

        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::String;

        Ok(readed + readed_1 + size)
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
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        let data_repr = self.0.data.iter().fold(String::new(), |data_repr, x| {
            data_repr.add(&format!(" {:02X}", x))
        });
        self.0.repr(&data_repr)
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{:?}", self.0.data), name)
    }

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
        if (readed + readed_1 + size) as usize > into.len() {
            return Err(Error::new(
                &format!(
                    "expected {} bytes, found `{}`",
                    (readed + readed_1 + size),
                    into.len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }
        let value =
            into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize].to_vec();
        self.0.data = value;
        self.0.number = index;
        self.0.type_ = FieldType::Bytes;

        Ok(readed + readed_1 + size)
    }
}

/// Filed with type StartGroup
/// TODO: change type
#[derive(Debug, Clone, PartialEq)]
pub struct StartGroupField(pub Field<i32>);

impl StartGroupField {
    fn new(name: String, number: u64, data: i32) -> Self {
        Self {
            0: Field::new(
                name,
                FieldLabel::Optional,
                FieldType::StartGroup,
                number,
                data,
            ),
        }
    }
}

impl Default for StartGroupField {
    fn default() -> Self {
        StartGroupField {
            0: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::StartGroup,
                number: 0,
                data: 0,
            },
        }
    }
}

impl FieldTrait for StartGroupField {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        self.0.repr(&format!("{:#x}", self.0.data))
    }

    fn to_str(&self, name: &str) -> String {
        self.0.to_str(&format!("{}", self.0.data), name)
    }

    fn serialize_into(&self, into: &mut Vec<u8>) {
        serialize_varint_into(
            generate_key(self.0.number, VariantTypeRaw::from(self.0.type_) as u8),
            into,
        );
        //serialize_varint_into(self.0.data as u64, into);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }

    fn deserialize(&mut self, into: &[u8]) -> Result<u64> {
        let (key, readed) = deserialize_varint(into)?;
        let (index, type_int) = parse_key(key);
        // Check Type if queal to `VariantTypeRaw::StartGroup`
        if type_int != VariantTypeRaw::StartGroup as u8 {
            return Err(Error::new(
                &format!(
                    "expected `{}` found `{}`",
                    VariantTypeRaw::StartGroup,
                    VariantTypeRaw::from(type_int)
                ),
                Some(ErrorType::IncorrectType),
            ));
        }
        // let (value, readed_x) = deserialize_varint(&into[readed as usize..])?;
        // if (value >> 0x32) != 0 {
        //     return Err(Error::new(
        //         "expected `Int32` found `U/Int64`",
        //         Some(ErrorType::IncorrectData),
        //     ));
        // }

        self.0.data = 0;
        self.0.number = index;
        self.0.type_ = FieldType::Int32;

        Ok(readed)
    }
}

pub struct FieldsVector {
    pub fields: Vec<Box<dyn FieldTrait>>,
}

impl Default for FieldsVector {
    fn default() -> Self {
        Self { fields: Vec::new() }
    }
}

/// Filed with type Embedded
pub struct EmbeddedField {
    pub field: Field<FieldsVector>,
    pub raw: Option<Vec<u8>>,
}

impl EmbeddedField {
    fn new(name: String, number: u64, data: FieldsVector) -> Self {
        Self {
            field: Field::new(name, FieldLabel::Optional, FieldType::Bytes, number, data),
            raw: None,
        }
    }
}

impl Default for EmbeddedField {
    fn default() -> Self {
        EmbeddedField {
            field: Field {
                name: "".to_string(),
                rule: FieldLabel::Optional,
                type_: FieldType::Bytes,
                number: 0,
                data: FieldsVector::default(),
            },
            raw: None,
        }
    }
}

impl FieldTrait for EmbeddedField {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn repr(&self) -> String {
        let raw = match &self.raw {
            None => "".to_string(),
            Some(data) => format!(
                "{:}",
                data.iter().fold(String::new(), |data_repr, x| {
                    data_repr.add(&format!(" {:02X}", x))
                })
            ),
        };

        let fields = match self.field.data.fields.len() > 0 {
            false => "".to_string(),
            true => format!(
                "{:}",
                self.field
                    .data
                    .fields
                    .iter()
                    .fold(String::new(), |data_repr, x| {
                        data_repr.add(&format!("\n\t{}", x.repr()))
                    })
            ),
        };

        self.field.repr(&format!("Raw <{}> {}", raw, fields))
    }

    fn to_str(&self, name: &str) -> String {
        let fields = match self.field.data.fields.len() > 0 {
            false => "".to_string(),
            true => format!(
                "{:}",
                self.field.data.fields.iter().enumerate().fold(
                    String::new(),
                    |data_repr, (i, x)| {
                        data_repr.add(&format!("\n\t{}", x.to_str(&format!("param{}", i))))
                    }
                )
            ),
        };

        format!(
            "message {name} {{\n{fields}\n}}\n
            {rule} {type} {name} = {number};",
            number = self.field.number,
            rule = self.field.rule,
            type = self.field.type_.to_str(),
            fields = fields,
            name = name
        )
    }

    fn serialize_into(&self, into: &mut Vec<u8>) {
        let mut embedded = Vec::new();
        self.field
            .data
            .fields
            .iter()
            .for_each(|x| x.serialize_into(&mut embedded));

        serialize_varint_into(
            generate_key(
                self.field.number,
                VariantTypeRaw::from(self.field.type_) as u8,
            ),
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
        if readed as usize >= into.len() {
            return Err(Error::new(
                &format!("insufficient amount of data to continue parsing"),
                Some(ErrorType::IncorrectData),
            ));
        }
        let (size, readed_1) = deserialize_varint(&into[readed as usize..])?;
        if (readed + readed_1 + size) as usize > into.len() {
            return Err(Error::new(
                &format!(
                    "expected {} bytes, found `{}`",
                    (readed + readed_1 + size),
                    into.len()
                ),
                Some(ErrorType::IncorrectData),
            ));
        }
        self.raw =
            Some(into[(readed + readed_1) as usize..(readed + readed_1 + size) as usize].to_vec());
        self.field.data = FieldsVector::default();
        self.field.number = index;
        self.field.type_ = FieldType::Embedded;

        Ok(readed + readed_1 + size)
    }
}

#[cfg(test)]
mod test {
    use crate::proto::field::*;

    #[test]
    fn serialization() {
        fn check<T: FieldTrait>(field: T, proto: &[u8]) {
            let proto_vec: Vec<u8> = field.serialize();
            assert_eq!(proto, &proto_vec);
        }
        // Check Int32
        check(
            Int32Field::new("".to_string(), 1, -0xFFFFFF),
            &[
                0x8, 0x81, 0x80, 0x80, 0xf8, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1,
            ],
        );
        // Check Int64
        check(
            Int64Field::new("".to_string(), 1, -0xFFFFFFFFFFFFFF),
            &[
                0x8, 0x81, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0xff, 0x1,
            ],
        );
        // Check UInt32
        check(
            UInt32Field::new("".to_string(), 1, 0x9FFFFFFF),
            &[0x8, 0xff, 0xff, 0xff, 0xff, 0x9],
        );
        // Check UInt64
        check(
            UInt64Field::new("".to_string(), 1, 0x9FFFFFFFFFFFFFFE),
            &[
                0x8, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x9f, 0x1,
            ],
        );
        // Check SInt32
        check(
            SInt32Field::new("".to_string(), 1, -0xFFFFFF),
            &[0x8, 0xfd, 0xff, 0xff, 0xf],
        );
        // Check SInt64
        check(
            SInt64Field::new("".to_string(), 1, -0xFFFFFFFFFFFFFF),
            &[0x8, 0xfd, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x1],
        );

        // Check String
        check(
            StringField::new("".to_string(), 0x10, "test value".to_string()),
            &[130, 1, 10, 116, 101, 115, 116, 32, 118, 97, 108, 117, 101],
        );

        check(
            StringField::new("".to_string(), 0x10, "".to_string()),
            &[130, 1, 0],
        );

        // Check Bytes
        check(
            BytesField::new("".to_string(), 0x10, "test value".as_bytes()),
            &[130, 1, 10, 116, 101, 115, 116, 32, 118, 97, 108, 117, 101],
        );
    }
}
