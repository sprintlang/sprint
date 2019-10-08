use assert_cmd::prelude::*;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process::Command;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Sprint Compiler", about = "Compiler for Sprint to Move IR.")]
struct Args {
    //File to be compiled.
    #[structopt(parse(from_os_str))]
    source_path: PathBuf,

    // Optional path to output file.
    #[structopt(parse(from_os_str))]
    output_path: Option<PathBuf>,
}

fn main() {
    let args = Args::from_args();

    let (source_path, output_path) = check_args(&args);

    let source = read_source(source_path);

    // TODO: Parse and Move code generation.
    // Currently the source file is written to output file as code generation has not been implemented.
    write_output(&output_path, source.as_bytes());
}

fn read_source(path: &PathBuf) -> String {
    let source_file =
        File::open(path).unwrap_or_else(|err| panic!("Unable to open file {:?}: {}", path, err));

    let mut buf_reader = BufReader::new(source_file);
    let mut source = String::new();

    buf_reader
        .read_to_string(&mut source)
        .unwrap_or_else(|err| panic!("Unable to read file {:?}: {}", path, err));
    source
}

fn write_output(path: &PathBuf, buf: &[u8]) {
    let mut move_file = File::create(path)
        .unwrap_or_else(|err| panic!("Unable to create file {:?}: {}", path, err));
    move_file
        .write_all(&buf)
        .unwrap_or_else(|err| panic!("Unable to write to file {:?}: {}", path, err));
}

// Checks for presence of output path and that file extensions are valid.
fn check_args(args: &Args) -> (&PathBuf, PathBuf) {
    let sprint_extension = "sprint";
    let source = &args.source_path;
    let extension = source
        .extension()
        .expect("Missing file extension on source file");
    if extension != sprint_extension {
        panic!(
            "Bad extension on source file {:?}, expected `{}`",
            extension, sprint_extension
        );
    }

    let output = create_output_path(&args);

    (source, output)
}

fn create_output_path(args: &Args) -> PathBuf {
    let mvir_extension = "mvir";
    let output_path = &args.output_path;
    let mut output = PathBuf::new();
    match output_path {
        Some(path) => {
            if path.extension() != Some(OsStr::new(mvir_extension)) {
                panic!(
                    "Output path must specify file with `{}` extension",
                    mvir_extension
                );
            }
            output.push(path);
        }
        None => {
            output.push(args.source_path.file_stem().unwrap());
            output.set_extension(mvir_extension);
        }
    };
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fails_with_no_args() {
        let mut cmd = Command::cargo_bin("sprintc").unwrap();
        cmd.assert().failure();
    }

    #[test]
    fn uses_source_stem_when_no_output_specified() {
        let args = Args {
            source_path: PathBuf::from("test.sprint"),
            output_path: None,
        };

        assert_eq!(create_output_path(&&args), PathBuf::from("test.mvir"));
    }

    #[test]
    fn uses_output_stem_when_specified() {
        let args = Args {
            source_path: PathBuf::from("test.sprint"),
            output_path: Some(PathBuf::from("output.mvir")),
        };

        assert_eq!(create_output_path(&args), PathBuf::from("output.mvir"));
    }
}
