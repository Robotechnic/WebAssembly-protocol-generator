use std::{collections::HashMap, fmt::Debug};

use crate::types::Types;

#[derive(Debug)]
pub enum ProtocolType {
    C,
    Typst,
}

#[derive(Debug)]
pub enum StructType {
    Struct,
    Protocol(ProtocolType),
}

pub struct Struct<'a> {
    type_: StructType,
    pub encoder: bool,
    pub decoder: bool,
    fields: HashMap<&'a str, Types>,
}

impl<'a> Struct<'a> {
    pub fn new(struct_type: StructType) -> Struct<'a> {
        Struct {
            type_: struct_type,
            encoder: false,
            decoder: false,
            fields: HashMap::new(),
        }
    }

    pub fn add_field(&mut self, name: &'a str, field_type: Types) {
        self.fields.insert(name, field_type);
    }

    pub fn get_type(&self) -> &StructType {
        &self.type_
    }

    pub fn fields(&self) -> std::collections::hash_map::Iter<'_, &str, Types> {
        self.fields.iter()
    }
}

impl<'a> Debug for Struct<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.type_ {
            StructType::Struct => {
                write!(f, "Struct {{")?;
            }
            StructType::Protocol(protocol_type) => {
                write!(f, "Protocol ")?;
                match protocol_type {
                    ProtocolType::C => {
                        write!(f, "C {{")?;
                    }
                    ProtocolType::Typst => {
                        write!(f, "Typst {{")?;
                    }
                }
            }
        }
        for (name, field) in &self.fields {
            write!(f, "\n\t{}: {:?}", name, field)?;
        }
        write!(f, "\n}}")
    }
}
