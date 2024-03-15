use wasmpg::{generator::{cgenerator, typstgenerator}, parser::ProtocolParser};


fn main() {
	let file = std::fs::read_to_string("exemples/struct.prot").unwrap();
    let result = ProtocolParser::parse_protocol(file.as_str());
	match result {
		Ok(protocol) => {
			match cgenerator::generate_protocol(".", &protocol) {
				Ok(_) => {
					println!("Generated C protocol");
				}
				Err(e) => {
					println!("Error: {:?}", e);
				}
			}
			match typstgenerator::generate_protocol(".", &protocol) {
				Ok(_) => {
					println!("Generated Typst protocol");
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
