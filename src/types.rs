use std::fmt::Debug;

pub enum Types {
    Int,
    Float,
    Bool,
    Char,
    String,
    Array(Box<Types>),
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
            _ => Types::Struct(type_str.to_string()),
        }
    }

    pub fn to_c(&self) -> String {
        match self {
            Types::Int => "int".to_string(),
            Types::Float => "float".to_string(),
            Types::Bool => "bool".to_string(),
            Types::Char => "char".to_string(),
            Types::String => "char*".to_string(),
            Types::Array(t) => format!("{} *", t.to_c()),
            Types::Struct(name) => name.to_string(),
        }
    }

	pub fn to_typst(&self) -> String {
		match self {
			Types::Int => "int".to_string(),
			Types::Float => "float".to_string(),
			Types::Bool => "bool".to_string(),
			Types::Char => "char".to_string(),
			Types::String => "string".to_string(),
			Types::Array(t) => format!("{}[]", t.to_typst()),
			Types::Struct(name) => name.to_string(),
		}
	}
}

impl Debug for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Types::Int => write!(f, "int"),
            Types::Float => write!(f, "float"),
            Types::Bool => write!(f, "bool"),
            Types::Char => write!(f, "char"),
            Types::String => write!(f, "string"),
            Types::Array(t) => write!(f, "{:?}[]", t),
            Types::Struct(name) => write!(f, "{}", name),
        }
    }
}
