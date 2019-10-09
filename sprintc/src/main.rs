use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

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

    let (source_path, output_path) = check_args(&args)?;

    let source = read_source(source_path)?;

    // TODO: Parse and Move code generation.
    // Currently the source file is written to output file as code generation has not been implemented.
    write_output(&output_path, source.as_bytes())?;

    Ok(())
}

// Checks for presence of output path and that file extensions are valid.
fn check_args(args: &Args) -> Result<(&PathBuf, PathBuf), String> {
    const SPRINT_EXTENSION: &str = "sprint";
    let source = &args.source_path;
    let extension = source.extension();

    match extension {
        Some(ext) => {
            if ext != SPRINT_EXTENSION {
                return Err(format!(
                    "Bad extension on source file {:?}, expected `{}`",
                    extension, SPRINT_EXTENSION
                ));
            }
        }
        None => {
            return Err("Missing file extension on source path".to_string());
        }
    }

    let output = create_output_path(&args)?;

    Ok((source, output))
}

fn create_output_path(args: &Args) -> Result<PathBuf, String> {
    const MVIR_EXTENSION: &str = "mvir";
    let output_path = &args.output_path;
    let mut output = PathBuf::new();
    match output_path {
        Some(path) => {
            if path.extension() != Some(OsStr::new(MVIR_EXTENSION)) {
                return Err(format!(
                    "Output path must specify file with `{}` extension",
                    MVIR_EXTENSION
                ));
            }
            output.push(path);
        }
        None => {
            output.push(args.source_path.file_stem().unwrap());
            output.set_extension(MVIR_EXTENSION);
        }
    };
    Ok(output)
}

fn read_source(path: &Path) -> Result<String, String> {
    let source_file =
        File::open(path).map_err(|err| format!("Unable to open file {:?}: {}", path, err))?;

    let mut buf_reader = BufReader::new(source_file);
    let mut source = String::new();

    buf_reader
        .read_to_string(&mut source)
        .map_err(|err| format!("Unable to read to file {:?}: {}", path, err))?;

    Ok(source)
}

fn write_output(path: &Path, buf: &[u8]) -> Result<(), String> {
    let mut move_file =
        File::create(path).map_err(|err| format!("Unable to create file {:?}: {}", path, err))?;

    move_file
        .write_all(&buf)
        .map_err(|err| format!("Unable to write to file {:?}: {}", path, err))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_output_path_no_output_specified() {
        let args = Args {
            source_path: PathBuf::from("test.sprint"),
            output_path: None,
        };

        assert_eq!(
            create_output_path(&args).unwrap(),
            PathBuf::from("test.mvir")
        );
    }

    #[test]
    fn create_output_path_output_specified() {
        let args = Args {
            source_path: PathBuf::from("test.sprint"),
            output_path: Some(PathBuf::from("output.mvir")),
        };

        assert_eq!(
            create_output_path(&args).unwrap(),
            PathBuf::from("output.mvir")
        );
    }
}
