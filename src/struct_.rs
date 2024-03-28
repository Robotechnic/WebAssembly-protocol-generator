use std::{collections::HashSet, fmt::Debug};

use crate::types::Types;

#[derive(Debug)]
pub enum ProtocolType {
    C,
    Typst,
	Bidirectional,
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
	fields_names: HashSet<&'a str>,
    fields: Vec<(&'a str, Types)>,
}

impl<'a> Struct<'a> {
    pub fn new(struct_type: StructType) -> Struct<'a> {
        Struct {
            type_: struct_type,
            encoder: false,
            decoder: false,
			fields_names: HashSet::new(),
            fields: Vec::new()
        }
    }

    pub fn add_field(&mut self, name: &'a str, field_type: Types) {
        self.fields.push((name, field_type));
		self.fields_names.insert(name);
    }

	pub fn has_field(&self, name: &str) -> bool {
		self.fields_names.contains(name)
	}

    pub fn get_type(&self) -> &StructType {
        &self.type_
    }

    pub fn fields(&self) -> std::slice::Iter<(&'a str, Types)> {
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
					ProtocolType::Bidirectional => {
						write!(f, "Bidirectional {{")?;
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
