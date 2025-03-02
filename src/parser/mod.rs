use crate::Protocol;
use crate::Types;
use crate::{ProtocolType, Struct, StructType};
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
                Rule::PROTOCOL_BIDIRECTIONAL => ProtocolType::Bidirectional,
                _ => unreachable!(),
            }
        }

        fn parse_fields<'a>(
            pair: Pair<'a, Rule>,
            struct_type: StructType,
            protocol: &Protocol,
			pos: pest::Span<'a>
        ) -> Result<Struct<'a>, Error<Rule>> {
            let mut fields = Struct::new(struct_type, pos);
            for pair in pair.into_inner() {
                let mut pair = pair.into_inner(); // get the block content
                let type_tok = pair.next().unwrap();
                let pos = type_tok.as_span();
                let mut field_type = Types::parse(type_tok.as_str());
                if let Types::Struct(ref name) = field_type {
                    if !protocol.has_struct(&name) {
                        return Err(Error::new_from_span(
                            ErrorVariant::CustomError {
                                message: format!("Struct \"{}\" not found", name),
                            },
                            pos,
                        ));
                    }
                }
                let name = pair.next().unwrap().as_str();
                if let Some(_) = pair.next() {
                    // we are in list mode
                    field_type = Types::Array(Box::new(field_type));
                }
                if fields.has_field(name) {
                    return Err(Error::new_from_span(
                        ErrorVariant::CustomError {
                            message: format!("Field \"{}\" already defined", name),
                        },
                        pos,
                    ));
                }
                fields.add_field(name, field_type, pos);
            }
            match fields.get_type() {
                StructType::Protocol(ProtocolType::C) => {
                    fields.decoder = true;
                }
                StructType::Protocol(ProtocolType::Typst) => {
                    fields.encoder = true;
                }
                _ => {}
            }
            Ok(fields)
        }

        fn parse_protocol(program: Pair<Rule>) -> Result<Protocol, Error<Rule>> {
            let mut protocol = Protocol::default();
            for declarations in program.into_inner() {
				let pos = declarations.as_span();
                match declarations.as_rule() {
                    Rule::STRUCT_DEF => {
                        let mut struct_def = declarations.into_inner();
                        let name = struct_def.next().unwrap().as_str();
						protocol.pre_add_struct(name, StructType::Struct, pos);
                        protocol.add_struct(
                            name,
                            parse_fields(struct_def.next().unwrap(), StructType::Struct, &protocol, pos)?,
                        ).map_err(|(msg, pos)| Error::new_from_span(
							ErrorVariant::CustomError { message: msg },
							pos,
						))?;
                    }
                    Rule::PROTOCOL_DEF => {
                        let mut protocol_def = declarations.into_inner();
                        let protocol_type = parse_protocol_type(protocol_def.next().unwrap());
                        let name = protocol_def.next().unwrap().as_str();
                        protocol.add_protocol(
                            name,
                            parse_fields(
                                protocol_def.next().unwrap(),
                                StructType::Protocol(protocol_type),
                                &protocol,
								pos,
                            )?,
                        ).map_err(|(msg, pos)| Error::new_from_span(
							ErrorVariant::CustomError { message: msg },
							pos,
						))?;
                    }
                    _ => unreachable!(),
                }
            }
            Ok(protocol)
        }

        parse_protocol(protocol)
    }
}
