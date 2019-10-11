use super::Span;
use nom::error::{convert_error, ErrorKind, ParseError, VerboseError};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub line: usize,
    pub column: usize,
    pub input: String,
    pub kind: ErrorKind,
}

impl Error {
    pub fn pretty(&self, original: &str) -> String {
        let error = VerboseError::<&str>::from_error_kind(&self.input, self.kind);
        convert_error(original, error)
    }
}

impl ParseError<Span<'_>> for Error {
    fn from_error_kind(input: Span, kind: ErrorKind) -> Self {
        Error {
            line: input.line as usize,
            column: input.get_column(),
            input: String::from(input.fragment),
            kind,
        }
    }

    fn append(_: Span, _: ErrorKind, other: Self) -> Self {
        other
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
        let error = Error::from_char(new, 'b');

        assert_eq!(error.line, 1);
        assert_eq!(error.column, 4);
    }
}
