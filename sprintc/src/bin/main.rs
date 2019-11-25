use sprintc::compile;
use structopt::StructOpt;

use std::{error::Error, path::PathBuf};

#[derive(Debug, StructOpt)]
#[structopt(name = "Sprint Compiler", about = "Compiler for Sprint to Move IR")]
pub struct Args {
    /// File to be compiled
    #[structopt(parse(from_os_str))]
    source_path: PathBuf,

    /// Optional path to output file
    #[structopt(parse(from_os_str))]
    output_path: Option<PathBuf>,

    /// Prints extra debugging output
    #[structopt(short, long)]
    verbose: bool,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    match compile(&args.source_path, &args.output_path, args.verbose) {
        Ok(path) => {
            println!(
                "Succesfully compiled {}!\nCompiled to {}.",
                args.source_path
                    .clone()
                    .into_os_string()
                    .into_string()
                    .unwrap(),
                path.to_str().unwrap()
            );
            Ok(())
        }
        Err(e) => Err(e),
    }
}
