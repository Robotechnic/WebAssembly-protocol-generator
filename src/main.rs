use std::{fs::metadata, process::exit};

use clap::Parser;
use wasmpg::{
    argparse::Argparse,
    generator::{cgenerator, typstgenerator},
    parser::ProtocolParser,
};

fn main() {
    let args = Argparse::parse();

    let file = match std::fs::read_to_string(args.input) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error: {}", e.to_string());
			exit(1);
        }
    };

    match metadata(&args.output_dir) {
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

    let result = ProtocolParser::parse_protocol(file.as_str());
    match result {
        Ok(protocol) => {
            match cgenerator::generate_protocol(&args.output_dir, &protocol) {
                Ok(_) => {
                    println!("Generated C protocol");
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            match typstgenerator::generate_protocol(&args.output_dir, &protocol) {
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
