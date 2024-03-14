use crate::Protocol;
use crate::{ProtocolType, Struct, StructType};
use crate::Types;
use pest::error::{Error, ErrorVariant};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser/protocol.pest"]
pub struct ProtocolParser;

impl ProtocolParser {
    pub fn parse_protocol(file: &str) -> Result<Protocol, Error<Rule>> {
        let protocol = ProtocolParser::parse(Rule::protocol, file)?.next().unwrap();

        use pest::iterators::Pair;
        fn parse_protocol_type(pair: Pair<Rule>) -> ProtocolType {
            match pair.into_inner().next().unwrap().as_rule() {
                Rule::PROTOCOL_C => ProtocolType::C,
                Rule::PROTOCOL_TYPST => ProtocolType::Typst,
                _ => unreachable!(),
            }
        }

        fn parse_fields<'a>(
            pair: Pair<'a, Rule>,
            struct_type: StructType,
			protocol: &Protocol,
            inside_struct: bool,
        ) -> Result<Struct<'a>, Error<Rule>> {
            let mut fields = Struct::new(struct_type);
			for pair in pair.into_inner() {
				let mut pair = pair.into_inner(); // get the block content
				let type_tok = pair.next().unwrap();
				let pos = type_tok.as_span();
				let mut field_type = Types::parse(type_tok.as_str());
				if let Types::Struct(ref name) = field_type {
					if !inside_struct {
						return Err(Error::new_from_span(ErrorVariant::CustomError { message: format!("You can't nest struct") }, pos));
					} else if !protocol.has_struct(&name) {
						return Err(Error::new_from_span(ErrorVariant::CustomError { message: format!("Struct \"{}\" not found", name) }, pos));
					}
				}
				let name = pair.next().unwrap().as_str();
				if let Some(_) = pair.next() { // we are in list mode
					field_type = Types::Array(Box::new(field_type));
				}
				fields.add_field(name, field_type);
			}
            Ok(fields)
        }

        fn parse_protocol(pair: Pair<Rule>) -> Result<Protocol, Error<Rule>> {
			let mut protocol = Protocol::default();
			for pair in pair.into_inner() {
				match pair.as_rule() {
					Rule::STRUCT_DEF => {
						let mut pair = pair.into_inner();
						let name = pair.next().unwrap().as_str();
						protocol.add_struct(
							name,
							parse_fields(pair.next().unwrap(), StructType::Struct, &protocol, false)?,
						);
					}
					Rule::PROTOCOL_DEF => {
						let mut pair = pair.into_inner();
						let protocol_type = parse_protocol_type(pair.next().unwrap());
						let name = pair.next().unwrap().as_str();
						protocol.add_protocol(
							name,
							parse_fields(
								pair.next().unwrap(),
								StructType::Protocol(protocol_type),
								&protocol,
								true,
							)?
						);
					}
					_ => unreachable!(),
				}
			};
			Ok(protocol)
        }

        parse_protocol(protocol)
    }
}
