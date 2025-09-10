use std::fmt::Debug;

/// Represents the different types that can be used in the protocol
#[derive(Clone)]
pub enum Types {
    Int,
    Float,
	Point,
    Bool,
    Char,
    String,
    Array(Box<Types>),
    Optional(Box<Types>),
    Struct(String),
}

impl Types {
    pub fn parse(type_str: &str) -> Types {
        match type_str {
            "int" => Types::Int,
            "float" => Types::Float,
            "bool" => Types::Bool,
            "char" => Types::Char,
            "string" => Types::String,
			"point" => Types::Point,
            _ => Types::Struct(type_str.to_string()),
        }
    }

	/// Check if the type is a struct or an array of structs
	pub fn is_struct(&self) -> bool {
		match self {
			Types::Struct(_) => true,
			Types::Array(t) => t.is_struct(),
            Types::Optional(t) => t.is_struct(),
			_ => false,
		}
	}

    pub fn to_c(&self, in_struct: bool) -> String {
        match self {
            Types::Int => "int".to_string(),
            Types::Float | Types::Point => "float".to_string(),
            Types::Bool => "bool".to_string(),
            Types::Char => "char".to_string(),
            Types::String => "char*".to_string(),
            Types::Array(t) => format!("{} *", t.to_c(in_struct)),
            Types::Struct(name) => if in_struct {
				format!("struct {}_t", name)
			} else {
				name.to_string()
			}
            Types::Optional(t) => format!("{} *", t.to_c(in_struct)),
        }
    }

	pub fn to_typst(&self) -> String {
		match self {
			Types::Int => "int".to_string(),
			Types::Float => "float".to_string(),
			Types::Point => "point".to_string(),
			Types::Bool => "bool".to_string(),
			Types::Char => "char".to_string(),
			Types::String => "string".to_string(),
			Types::Array(t) => format!("{}[]", t.to_typst()),
			Types::Struct(name) => name.to_string(),
            Types::Optional(t) => format!("{}", t.to_typst()),
		}
	}
}

impl Debug for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Int => write!(f, "int"),
            Types::Float => write!(f, "float"),
			Types::Point => write!(f, "point"),
            Types::Bool => write!(f, "bool"),
            Types::Char => write!(f, "char"),
            Types::String => write!(f, "string"),
            Types::Array(t) => write!(f, "{:?}[]", t),
            Types::Struct(name) => write!(f, "{}", name),
            Types::Optional(t) => write!(f, "{:?}?", t),
        }
    }
}
