use std::{collections::{HashMap, HashSet}, fmt::Debug};

use pest::Span;

use crate::{
    struct_::{ProtocolType, StructType},
    types::Types,
    Struct,
};

/// A struct that contains all the structs and protocols defined in the protocol file
pub struct Protocol<'a> {
	structs_order: Vec<&'a str>,
    structs: HashMap<&'a str, Struct<'a>>,
    protocols: HashMap<&'a str, Struct<'a>>,
}

impl<'a> Default for Protocol<'a> {
    fn default() -> Protocol<'a> {
        Protocol {
			structs_order: Vec::new(),
            structs: HashMap::new(),
            protocols: HashMap::new(),
        }
    }
}

impl<'a> Protocol<'a> {
	/// Check for circular dependencies in the structs children types
	fn check_circular_dependencies(&self, struct_: &Struct<'a>, parents: &HashSet<&str>) -> Result<(), (String, Span<'a>)> {
		for (_, t, pos) in struct_.iter() {
			match t {
				Types::Struct(name) => {
					if parents.contains(name.as_str()) {
						return Err((
							format!("Circular dependency detected: {} is its own parent", name),
							pos.clone()
						))
					}
					if let Some(s) = self.structs.get(name.as_str()) {
						let mut parents = parents.clone();
						parents.insert(name.as_str());
						self.check_circular_dependencies(s, &parents)?;
					}
				}
				_ => {}
			}
		}
		Ok(())
	}
	
	pub fn pre_add_struct(&mut self, name: &'a str, struct_type : StructType, pos: pest::Span<'a>) {
		self.structs.insert(name, Struct::new(struct_type, pos));
		self.structs_order.push(name);
	}

    pub fn add_struct(&mut self, name: &'a str, struct_: Struct<'a>) -> Result<(), (String, Span<'a>)> {
		let mut set = HashSet::new();
		set.insert(name);
		self.check_circular_dependencies(&struct_, &set)?;
        if self.structs.insert(name, struct_).is_none() {
			self.structs_order.push(name);
		}
		Ok(())
    }


	/// Add a new protocol to the program
    pub fn add_protocol(&mut self, name: &'a str, protocol: Struct<'a>) -> Result<(), (String, Span<'a>)> {
		for (_, t, pos) in protocol.iter() {
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

	fn update_children_encoding_type(&mut self, name: &str) {
		let structs = self.structs.get(name).unwrap();
		let encoder = structs.encoder;
		let decoder = structs.decoder;
		let structs = structs.iter().filter(|(_, t, _)| t.is_struct()).map(|(_, t, _)| t.clone()).collect::<Vec<_>>();
		for t in structs {
			match t {
				Types::Struct(name) => {
					let s = self.structs.get_mut(name.as_str()).unwrap();
					s.encoder |= encoder;
					s.decoder |= decoder;
					self.update_children_encoding_type(name.as_str());
				}
				Types::Array(t) => {
					if let Types::Struct(name) = t.as_ref() {
						let s = self.structs.get_mut(name.as_str()).unwrap();
						s.encoder |= encoder;
						s.decoder |= decoder;
						self.update_children_encoding_type(name.as_str());
					}
				}
				_ => {}
			}
		}
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
			self.update_children_encoding_type(name);
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

	pub fn ordered_structs(&self) -> impl Iterator<Item = (&'a str, &Struct<'a>)> {
		self.structs_order.iter().map(|name| (*name, self.structs.get(name).unwrap()))
	}

    pub fn protocols(&self) -> std::collections::hash_map::Iter<'_, &str, Struct<'a>> {
        self.protocols.iter()
    }
}

impl<'a> Debug for Protocol<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Protocol {{")?;
        for (name, struct_) in &self.structs {
            write!(f, "\n{}: {:?} ({},{})", name, struct_, struct_.encoder, struct_.decoder)?;
        }
        for (name, protocol) in &self.protocols {
            write!(f, "\n{}: {:?}", name, protocol)?;
        }
        write!(f, "\n}}")
    }
}
