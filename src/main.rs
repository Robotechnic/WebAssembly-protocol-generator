use wasmpg::parser::ProtocolParser;


fn main() {
	let file = std::fs::read_to_string("exemples/struct.prot").unwrap();
    let result = ProtocolParser::parse_protocol(file.as_str());
	match result {
		Ok(protocol) => {
			println!("{:?}", protocol);
		}
		Err(e) => {
			println!("Error: {:?}", e);
		}
	}
}
