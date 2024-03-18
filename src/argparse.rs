use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "")]
#[command(about="", long_about=None)]
pub struct Argparse {
    pub input: String,
    #[arg(short, default_value = ".")]
    pub output_dir: String,
}
