// use core::fmt;
use crate::proto::field::{Field, FieldTrait};

/// Protobuf syntax
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Syntax {
    /// Protobuf syntax 2(default)
    Proto2,
    /// Protobuf syntax 3
    Proto3,
}

/// Protobuf message
//#[derive(Debug, Clone)]
pub struct Message {
    /// Message name
    pub name: String,
    /// List of fields
    pub fields: Vec<Box<dyn FieldTrait>>,
}

impl Message {
    pub fn new(name: String, fields: Option<Vec<Box<dyn FieldTrait>>>) -> Self {
        Message {
            name: name,
            fields: match fields {
                Some(x) => x,
                None => Vec::new(),
            },
        }
    }

    pub fn serialize_into(&self, into: &mut Vec<u8>) {
       self.fields.iter().for_each(|x| x.serialize_into(into));
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut gen = Vec::new();
        self.serialize_into(&mut gen);
        gen
    }
}
