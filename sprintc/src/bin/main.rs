use sprintc::compile;
use structopt::StructOpt;

use std::{error::Error, path::PathBuf};

#[derive(StructOpt, Debug)]
#[structopt(name = "Sprint Compiler", about = "Compiler for Sprint to Move IR")]
pub struct Args {
    /// File to be compiled
    #[structopt(parse(from_os_str))]
    pub source_path: PathBuf,

    /// Optional path to output file
    #[structopt(parse(from_os_str))]
    pub output_path: Option<PathBuf>,

    /// Prints extra debugging output
    #[structopt(short, long)]
    pub verbose: bool,

    /// Checks program without code generation
    #[structopt(short, long)]
    pub check: bool,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    let args = sprintc::CompileArgs {
        source_path: args.source_path,
        output_path: args.output_path,
        verbose: args.verbose,
        check: args.check,
    };

    match compile(&args) {
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
