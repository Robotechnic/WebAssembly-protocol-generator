use std::{fs::metadata, process::exit};

use clap::Parser;
use wasmpg::{
    argparse::Argparse,
    generator::{cgenerator, typstgenerator},
    parser::ProtocolParser,
};

fn check_folder(path: &str) {
	match metadata(path) {
		Ok(metadata) => {
			if !metadata.is_dir() {
				eprintln!("Error: Output directory is not a directory");
				exit(1);
			}
			if metadata.permissions().readonly() {
				eprintln!("Error: Output directory is not writable");
				exit(1);
			}
		}
		Err(e) => {
			eprintln!("Error: Invalid output directory {}", e.to_string());
			exit(1);
		}
	}
}

fn generate_protocols(c_folder: String, protocol: wasmpg::protocol::Protocol<'_>, typst_folder: String) {
	match cgenerator::generate_protocol(&c_folder, &protocol) {
		Ok(_) => {
			println!("Generated C protocol");
		}
		Err(e) => {
			println!("Error: {:?}", e);
		}
	}
	match typstgenerator::generate_protocol(&typst_folder, &protocol) {
		Ok(_) => {
			println!("Generated Typst protocol");
		}
		Err(e) => {
			println!("Error: {:?}", e);
		}
	}
}

fn main() {
    let args = Argparse::parse();

    let file = match std::fs::read_to_string(args.input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
			exit(1);
        }
    };
	
	if args.check {
		match ProtocolParser::parse_protocol(file.as_str()) {
			Ok(_) => {
				println!("Protocol is valid");
			}
			Err(e) => {
				println!("Error: {:?}", e);
			}
		}
		return;
	}

	let c_folder = if let Some(c_folder) = args.c_output {
		c_folder
	} else if let Some(output_dir) = args.output_dir.clone() {
		output_dir
	} else {
		".".to_string()
	};

	let typst_folder = if let Some(typst_folder) = args.typst_output {
		typst_folder
	} else if let Some(output_dir) = args.output_dir {
		output_dir
	} else {
		".".to_string()
	};

	check_folder(&c_folder);
	check_folder(&typst_folder);
    let result = ProtocolParser::parse_protocol(file.as_str());
    match result {
        Ok(protocol) => {
			if args.check {
				println!("Protocol is valid");
			} else {
            	generate_protocols(c_folder, protocol, typst_folder);
			}
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

