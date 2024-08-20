use std::{collections::HashSet, fmt::Debug};

use crate::types::Types;


#[derive(Debug)]
/// Each protocol can be of type C, Typst or Bidirectional
/// C: He protocol is encoded in Typst and decoded in C
/// Typst: The protocol is encoded in C and decoded in Typst
/// Bidirectional: The protocol is encoded and decoded in both Typst and C
pub enum ProtocolType {
    C,
    Typst,
	Bidirectional,
}

#[derive(Debug)]
/// Struct and protocols are represented the same way so this enum is used to differentiate between them
/// Struct: A struct used in the protocol, it's the same as a struct in C
/// Protocol: A protocol definition
pub enum StructType {
    Struct,
    Protocol(ProtocolType),
}

/// Used to represent a struct or a protocol in the protocol file
pub struct Struct<'a> {
    type_: StructType,
	pos: pest::Span<'a>,
    pub encoder: bool,
    pub decoder: bool,
	fields_names: HashSet<&'a str>,
	// fields are stored in a vector of tuples (name, type) because the order matters
    fields: Vec<(&'a str, Types, pest::Span<'a>)>,
}

impl<'a> Struct<'a> {
    pub fn new(struct_type: StructType, pos: pest::Span<'a> ) -> Struct<'a> {
        Struct {
            type_: struct_type,
			pos,
            encoder: false,
            decoder: false,
			fields_names: HashSet::new(),
            fields: Vec::new()
        }
    }

    pub fn add_field(&mut self, name: &'a str, field_type: Types, pos: pest::Span<'a>) {
        self.fields.push((name, field_type, pos));
		self.fields_names.insert(name);
    }

	pub fn has_field(&self, name: &str) -> bool {
		self.fields_names.contains(name)
	}

    pub fn get_type(&self) -> &StructType {
        &self.type_
    }

    pub fn fields(&self) -> std::slice::Iter<(&'a str, Types, pest::Span<'a>)> {
        self.fields.iter()
    }

	pub fn get_pos(&self) -> pest::Span<'a> {
		self.pos
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
        for (name, field, _) in &self.fields {
            write!(f, "\n\t{}: {:?}", name, field)?;
        }
        write!(f, "\n}}")
    }
}
