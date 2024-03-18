use std::fs::metadata;

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
            println!("Error: {:?}", e);
            return;
        }
    };

    match metadata(&args.output_dir) {
        Ok(metadata) => {
            if !metadata.is_dir() {
                println!("Error: Output directory is not a directory");
                return;
            }
            if metadata.permissions().readonly() {
                println!("Error: Output directory is not writable");
                return;
            }
        }
        Err(e) => {
            println!("Error: {}", e.to_string());
            return;
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
