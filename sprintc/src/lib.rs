use sprint_move::generate;
use sprint_parser::parser;
use std::{
    borrow::Cow,
    error::Error,
    ffi::OsStr,
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

const MVIR_EXTENSION: &str = "mvir";
const SPRINT_EXTENSION: &str = "sprint";

pub fn compile(
    source: &PathBuf,
    output: &Option<PathBuf>,
    verbose: bool,
) -> Result<(), Box<dyn Error>> {
    let (source_path, output_path) = check_args(source, output)?;

    let source = read_source(source_path)?;

    let ast = parser::contract(&source).map_err(|err| {
        eprint!("{}", err.pretty(&source));
        format!("Unable to parse file `{}`", source_path.display())
    })?;

    if verbose {
        println!("{:#?}", ast);
    }

    let output = generate(&ast);
    write_output(&output_path, output.as_bytes())?;

    Ok(())
}

// Checks for presence of output path and that file extensions are valid.
fn check_args<'a>(
    source: &'a PathBuf,
    output: &'a Option<PathBuf>,
) -> Result<(&'a Path, Cow<'a, Path>), String> {
    let extension = source.extension();

    match extension {
        Some(extension) => {
            if extension != SPRINT_EXTENSION {
                // to_str() returns None if the OsStr is not valid Unicode.
                let extension = extension
                    .to_str()
                    .ok_or("Source path is not valid unicode")?;

                return Err(format!(
                    "Incorrect extension on source file: got `{}`, expected `{}`",
                    extension, SPRINT_EXTENSION
                ));
            }
        }
        None => {
            return Err(String::from("Missing file extension on source path"));
        }
    }

    let output = create_output_path(source, output)?;

    Ok((source, output))
}

fn create_output_path<'a>(
    source_path: &PathBuf,
    output_path: &'a Option<PathBuf>,
) -> Result<Cow<'a, Path>, String> {
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

            output.push(source_path.file_stem().unwrap());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_output_path_no_output_specified() {
        assert_eq!(
            create_output_path(&PathBuf::from("test.sprint"), &None).unwrap(),
            PathBuf::from("test.mvir")
        );
    }

    #[test]
    fn create_output_path_output_specified() {
        assert_eq!(
            create_output_path(
                &PathBuf::from("test.sprint"),
                &Some(PathBuf::from("output.mvir"))
            )
            .unwrap(),
            PathBuf::from("output.mvir")
        );
    }
}
