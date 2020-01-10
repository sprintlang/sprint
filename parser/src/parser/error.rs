use super::Span;
use crate::ast::Kind;
use nom::error::{ErrorKind, ParseError};

#[derive(PartialEq, Debug)]
pub struct Error<'a> {
    pub nom_error: Option<NomError<'a>>,
    pub sprint_error: Option<SprintError<'a>>,
}

#[derive(PartialEq, Debug)]
pub struct NomError<'a> {
    pub line: usize,
    pub column: usize,
    pub input: &'a str,
    pub kind: ErrorKind,
}

#[derive(Clone, PartialEq, Debug)]
pub enum SprintError<'a> {
    TypeError(&'a str, Box<SprintError<'a>>),
    MismatchedKinds(Kind, Kind),
    UnknownIdentifierError(&'a str, Kind),
    DuplicateDefinitionError(&'a str),
    InvalidNumberArgsError,
    UndefinedMainError,
}

impl<'a> Error<'a> {
    pub fn pretty(&self, original: &str) -> String {
        let nom_error = match &self.nom_error {
            Some(err) => err.pretty(original),
            None => String::from(""),
        };
        let sprint_error = match &self.sprint_error {
            Some(err) => err.clone().pretty(),
            None => String::from(""),
        };

        format!("{}{}\n", sprint_error, nom_error)
    }

    pub fn from_sprint_error(sprint_error: SprintError<'a>, input: Option<Span<'a>>) -> Self {
        match input {
            Some(span) => Error {
                nom_error: Some(NomError::from_span(span)),
                sprint_error: Some(sprint_error),
            },
            None => Error {
                nom_error: None,
                sprint_error: Some(sprint_error),
            },
        }
    }
}

impl<'a> NomError<'a> {
    pub fn pretty(&self, original: &str) -> String {
        let line = self.line;
        let code = print_code_line(original, line);
        if code.is_empty() {
            code
        } else {
            format!("\nOn line {}: \n\t{}", line, code)
        }
    }

    fn from_span(input: Span<'a>) -> Self {
        NomError {
            line: input.line as usize,
            column: input.get_column(),
            input: input.fragment,
            // nom ErrorKind does not allow Custom or Default ErrorKinds.
            kind: ErrorKind::Tag,
        }
    }
}

impl<'a> SprintError<'a> {
    pub fn pretty(self) -> String {
        match self {
            Self::TypeError(definition, mismatched_kinds) => format!(
                "Type Error: From definition of \"{}\" {} ",
                definition,
                mismatched_kinds.pretty()
            ),
            Self::MismatchedKinds(actual, expected) => {
                format!("expected {}, got {}", actual, expected)
            }
            Self::UnknownIdentifierError(id, kind) => {
                format!("Unknown identifier: {} :: {}", id, kind)
            }
            Self::InvalidNumberArgsError => {
                String::from("Invalid number of arguments in primitive application")
            }
            Self::DuplicateDefinitionError(name) => {
                format!("Duplicate definition for \"{}\"", name)
            }
            Self::UndefinedMainError => {
                String::from("No valid definition of the \"main\" contract was found")
            }
        }
    }
}

impl<'a> ParseError<Span<'a>> for Error<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        Error {
            nom_error: Some(NomError::from_error_kind(input, kind)),
            sprint_error: None,
        }
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<'a> ParseError<Span<'a>> for NomError<'a> {
    fn from_error_kind(input: Span<'a>, kind: ErrorKind) -> Self {
        NomError {
            line: input.line as usize,
            column: input.get_column(),
            input: input.fragment,
            kind,
        }
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
    }
}

fn print_code_line(input: &str, line: usize) -> String {
    let lines: std::vec::Vec<String> = input.lines().map(String::from).collect();
    if lines.is_empty() {
        "".to_string()
    } else {
        // -1 to compensate for offset between line and index numbering.
        lines[line - 1].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::InputTake;
    use nom_locate::LocatedSpan;

    #[test]
    fn error_from_span() {
        let original = LocatedSpan::new("foo bar");
        let (new, _) = original.take_split(3);
        let error = NomError::from_char(new, 'b');

        assert_eq!(error.line, 1);
        assert_eq!(error.column, 4);
    }
}
