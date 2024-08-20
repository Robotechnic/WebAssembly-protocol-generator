use std::{collections::HashMap, fmt::Debug};

use pest::Span;

use crate::{
    struct_::{ProtocolType, StructType},
    types::Types,
    Struct,
};

/// A struct that contains all the structs and protocols defined in the protocol file
pub struct Protocol<'a> {
    structs: HashMap<&'a str, Struct<'a>>,
    protocols: HashMap<&'a str, Struct<'a>>,
}

impl<'a> Default for Protocol<'a> {
    fn default() -> Protocol<'a> {
        Protocol {
            structs: HashMap::new(),
            protocols: HashMap::new(),
        }
    }
}

impl<'a> Protocol<'a> {
    pub fn add_struct(&mut self, name: &'a str, struct_: Struct<'a>) {
        self.structs.insert(name, struct_);
    }

	/// Add a new protocol to the program
    pub fn add_protocol(&mut self, name: &'a str, protocol: Struct<'a>) -> Result<(), (String, Span<'a>)> {
		for (_, t, pos) in protocol.fields() {
			match t {
				Types::Struct(name) => {
					self.set_struct_encoding_type(name, pos, &protocol)?;
				}
				Types::Array(t) => {
					if let Types::Struct(name) = t.as_ref() {
						self.set_struct_encoding_type(name, pos, &protocol)?;
					}
				}
				_ => {}
			}
		}
        self.protocols.insert(name, protocol);
		Ok(())
    }

	/// Check the protocol type and set the encoding type of the struct accordingly
    fn set_struct_encoding_type(&mut self, name: &String, pos: &Span<'a>, parent_protocol: &Struct<'a>) -> Result<(), (String, Span<'a>)> {
        if !self.structs.contains_key(name.as_str()) {
			return Err((format!("Struct \"{}\" does not exist", name),
				pos.clone(),
			));
        }
		if let StructType::Protocol(parent_type) = parent_protocol.get_type() {
			let s = self.structs.get_mut(name.as_str()).unwrap();
			match parent_type {
				ProtocolType::C => {
					s.decoder = true;
				}
				ProtocolType::Typst => {
					s.encoder = true;
				}
				ProtocolType::Bidirectional => {
					s.encoder = true;
					s.decoder = true;
				}
			}
		} else {
			unreachable!();
		}
		Ok(())
    }

    pub fn has_protocol(&self, name: &str) -> bool {
        self.protocols.contains_key(name)
    }

    pub fn has_struct(&self, name: &str) -> bool {
        self.structs.contains_key(name)
    }

    pub fn structs(&self) -> std::collections::hash_map::Iter<'_, &str, Struct<'a>> {
        self.structs.iter()
    }

    pub fn protocols(&self) -> std::collections::hash_map::Iter<'_, &str, Struct<'a>> {
        self.protocols.iter()
    }
}

impl<'a> Debug for Protocol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Protocol {{")?;
        for (name, struct_) in &self.structs {
            write!(f, "\n{}: {:?}", name, struct_)?;
        }
        for (name, protocol) in &self.protocols {
            write!(f, "\n{}: {:?}", name, protocol)?;
        }
        write!(f, "\n}}")
    }
}
