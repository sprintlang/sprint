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

pub struct CompileArgs {
    /// File to be compiled
    pub source_path: PathBuf,
    /// Optional path to output file
    pub output_path: Option<PathBuf>,
    /// Prints extra debugging output
    pub verbose: bool,
    /// Checks program without code generation
    pub check: bool,
}

pub fn compile<'a>(args: &'a CompileArgs) -> Result<Cow<'a, Path>, Box<dyn Error>> {
    let (source_path, output_path) = check_args(args)?;

    let source = read_source(source_path)?;

    let ast = parser::contract(&source).map_err(|err| {
        eprint!("{}", err.pretty(&source));
        format!("Unable to parse file `{}`", source_path.display())
    })?;

    if args.verbose {
        for definition in &ast {
            let name = definition.variable.name;
            println!("{} :: {}", name, definition.variable.kind);
            println!("{} = {:#?}", name, definition.expression);
        }
    }

    if !args.check {
        let output = generate(&ast);
        write_output(&output_path, output.as_bytes())?;
    }

    Ok(output_path)
}

// Checks for presence of output path and that file extensions are valid.
fn check_args(args: &CompileArgs) -> Result<(&Path, Cow<Path>), String> {
    let source = &args.source_path;
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

    let output = create_output_path(&args)?;

    Ok((source, output))
}

fn create_output_path(args: &CompileArgs) -> Result<Cow<Path>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_output_path_no_output_specified() {
        let args = CompileArgs {
            source_path: PathBuf::from("test.sprint"),
            output_path: None,
            verbose: false,
            check: false,
        };

        assert_eq!(
            create_output_path(&args).unwrap(),
            PathBuf::from("test.mvir")
        );
    }

    #[test]
    fn create_output_path_output_specified() {
        let args = CompileArgs {
            source_path: PathBuf::from("test.sprint"),
            output_path: Some(PathBuf::from("output.mvir")),
            verbose: false,
            check: false,
        };

        assert_eq!(
            create_output_path(&args).unwrap(),
            PathBuf::from("output.mvir")
        );
    }
}
