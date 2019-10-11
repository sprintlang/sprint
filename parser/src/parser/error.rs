use nom::{
    error::{convert_error, ErrorKind, ParseError, VerboseError},
    AsBytes,
};
use nom_locate::LocatedSpan;

#[derive(Debug)]
pub struct Err {
    line: usize,
    column: usize,
    input: String,
    kind: ErrorKind,
}

impl Err {
    pub fn pretty<I: ToString>(&self, original: I) -> String {
        let input = self.input.to_string();
        let error = VerboseError::<&str>::from_error_kind(&input, self.kind);
        convert_error(&original.to_string(), error)
    }
}

impl<I: AsBytes + ToString> ParseError<LocatedSpan<I>> for Err {
    fn from_error_kind(input: LocatedSpan<I>, kind: ErrorKind) -> Self {
        Err {
            line: input.line as usize,
            column: input.get_column(),
            input: input.to_string(),
            kind,
        }
    }

    fn append(_: LocatedSpan<I>, _: ErrorKind, other: Self) -> Self {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::InputTake;

    #[test]
    fn error_from_span() {
        let original = LocatedSpan::new("foo bar");
        let (new, _) = original.take_split(3);
        let error = Err::from_char(new, 'b');

        assert_eq!(error.line, 1);
        assert_eq!(error.column, 4);
    }
}
