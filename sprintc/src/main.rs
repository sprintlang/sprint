use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use structopt::StructOpt;

const MVIR_EXTENSION: &str = "mvir";
const SPRINT_EXTENSION: &str = "sprint";

#[derive(Debug, StructOpt)]
#[structopt(name = "Sprint Compiler", about = "Compiler for Sprint to Move IR.")]
struct Args {
    // File to be compiled.
    #[structopt(parse(from_os_str))]
    source_path: PathBuf,

    // Optional path to output file.
    #[structopt(parse(from_os_str))]
    output_path: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    let (source_path, output_path) = check_args(&args);

    let source = read_source(source_path);

    let ast = parser::contract(&source).map_err(|err| {
        eprint!("{}", err.pretty(&source));
        format!("Unable to parse file `{}`", source_path.display())
    })?;

    // TODO: Move code generation.
    // Currently the source file is written to output file as code generation has not been implemented.
    write_output(&output_path, source.as_bytes());
}

fn create_output_path(args: &Args) -> Result<Cow<Path>, String> {
    let output_path = &args.output_path;

    match output_path {
        Some(path) => {
            if path.extension() != Some(OsStr::new(MVIR_EXTENSION)) {
                return Err(format!(
                    "Output path must specify file with `{}` extension",
                    MVIR_EXTENSION
                ));
            }

            Ok(path.into())
        }
        None => {
            let mut output = PathBuf::new();

            output.push(args.source_path.file_stem().unwrap());
            output.set_extension(MVIR_EXTENSION);

            Ok(output.into())
        }
    }
}

fn read_source(path: &Path) -> Result<String, String> {
    let source_file = File::open(path)
        .map_err(|err| format!("Unable to open file `{}`: {}", path.display(), err))?;

    let mut buf_reader = BufReader::new(source_file);
    let mut source = String::new();

    buf_reader
        .read_to_string(&mut source)
        .map_err(|err| format!("Unable to read to file `{}`: {}", path.display(), err))?;

    Ok(source)
}

fn write_output(path: &Path, buf: &[u8]) -> Result<(), String> {
    let mut move_file = File::create(path)
        .map_err(|err| format!("Unable to create file `{}`: {}", path.display(), err))?;

    move_file
        .write_all(&buf)
        .map_err(|err| format!("Unable to write to file `{}`: {}", path.display(), err))?;

    Ok(())
}

// Checks for presence of output path and that file extensions are valid.
fn check_args(args: &Args) -> (&PathBuf, PathBuf) {
    let sprint_extension = "sprint";
    let mvir_extension = "mvir";
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

    #[test]
    fn create_output_path_output_specified() {
        let args = Args {
            source_path: PathBuf::from("test.sprint"),
            output_path: Some(PathBuf::from("output.mvir")),
        };

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
            output.push(source.file_stem().unwrap());
            output.set_extension(mvir_extension);
        }
    };
    (source, output)
}

#[cfg(test)]
mod tests {
    use super::*;

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
