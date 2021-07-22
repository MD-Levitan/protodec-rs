// use core::fmt;
use crate::parser::field::{Field, FieldTrait};

/// Protobuf syntax
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Syntax {
    /// Protobuf syntax 2(default)
    Proto2,
    /// Protobuf syntax 3
    Proto3,
}

/// Protobuf message
#[derive(Debug, Clone)]
pub struct Message<T>
where
    T: FieldTrait,
{
    /// Message name
    pub name: String,
    /// List of fields
    pub fields: Vec<T>,
}

// #[derive(Serialize, Deserialize)]
// pub struct Variant {
//     /// Type of raw variant
//     raw_variant_type: VariantTypeRaw,
//     /// Type variant
//     variant_type:
// }
