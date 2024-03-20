use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "")]
#[command(about="", long_about=None)]
pub struct Argparse {
    pub input: String,
	/// Output directory for generated files
    #[arg(short)]
    pub output_dir: Option<String>,
	/// Output directory for generated C files, overrides output_dir
	#[arg(short, conflicts_with("output_dir"), requires("typst_output"))]
	pub c_output: Option<String>,
	/// Output directory for generated Typst files, overrides output_dir
	#[arg(short, conflicts_with("output_dir"), requires("c_output"))]
	pub typst_output: Option<String>,

	/// Check if the input file is a valid protocol file
	#[arg(long, action, conflicts_with("output_dir"), conflicts_with("c_output"), conflicts_with("typst_output"))]
	pub check: bool
}
