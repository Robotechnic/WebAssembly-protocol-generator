use wasmpg::{generator::cgenerator::generate_protocol, parser::ProtocolParser};


fn main() {
	let file = std::fs::read_to_string("exemples/struct.prot").unwrap();
    let result = ProtocolParser::parse_protocol(file.as_str());
	match result {
		Ok(protocol) => {
			match generate_protocol(".", protocol) {
				Ok(_) => {
					println!("Generated protocol");
				}
				Err(e) => {
					println!("Error: {:?}", e);
				}
			}
		}
		Err(e) => {
			println!("Error: {:?}", e);
		}
	}
}
