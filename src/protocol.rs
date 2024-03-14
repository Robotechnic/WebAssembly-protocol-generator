use std::{collections::HashMap, fmt::Debug};

use crate::Struct;

pub struct Protocol<'a> {
	structs: HashMap<&'a str, Struct<'a>>,
	protocols: HashMap<&'a str, Struct<'a>>
}

impl<'a> Default for Protocol<'a> {
	fn default() -> Protocol<'a> {
		Protocol {
			structs: HashMap::new(),
			protocols: HashMap::new()
		}
	}
}

impl<'a> Protocol<'a> {
	pub fn add_struct(&mut self, name: &'a str, struct_: Struct<'a>) {
		self.structs.insert(name, struct_);
	}

	pub fn add_protocol(&mut self, name: &'a str, protocol: Struct<'a>) {
		self.protocols.insert(name, protocol);
	}

	pub fn has_protocol(&self, name: &str) -> bool {
		self.protocols.contains_key(name)
	}

	pub fn has_struct(&self, name: &str) -> bool {
		self.structs.contains_key(name)
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