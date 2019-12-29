use super::IResult;
use nom::{
    character::complete::char,
    character::complete::multispace0,
    error::ParseError,
    multi::{count, many1_count},
    sequence::{delimited, terminated},
    AsBytes, AsChar, InputIter, InputTakeAtPosition, Slice,
};
use nom_locate::LocatedSpan;
use std::ops::RangeFrom;

pub fn span<'a, I, O, F>(f: F) -> impl Fn(I) -> IResult<'a, I, O>
where
    I: AsBytes,
    F: Fn(LocatedSpan<I>) -> IResult<'a, LocatedSpan<I>, O>,
{
    move |input: I| {
        let input = LocatedSpan::new(input);
        f(input).map(|(input, output)| (input.fragment, output))
    }
}

pub fn padding0<I, O, E, F>(f: F) -> impl Fn(I) -> nom::IResult<I, O, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
    F: Fn(I) -> nom::IResult<I, O, E>,
{
    delimited(multispace0, f, multispace0)
}

pub fn brackets1<I, O, E, F>(f: F) -> impl Fn(I) -> nom::IResult<I, O, E>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + PartialEq,
    <I as InputIter>::Item: AsChar,
    E: ParseError<I>,
    F: Fn(I) -> nom::IResult<I, O, E> + Copy,
{
    move |input: I| {
        let (input, brackets) = many1_count(char('('))(input)?;
        terminated(f, count(char(')'), brackets))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::super::{
        error::{Error, NomError},
        Span,
    };
    use super::*;
    use nom::{
        bytes::complete::tag, character::complete::char, error::ErrorKind, sequence::tuple, Err,
    };

    fn parser(input: &str) -> nom::IResult<&str, &str> {
        tag("abc")(input)
    }

    fn parser_span(input: Span) -> IResult<Span, &str> {
        let (input, _) = tuple((char('a'), char('b'), char('c')))(input)?;
        Ok((input, "abc"))
    }

    #[test]
    fn parse_span() {
        assert_eq!(span(parser_span)("abc"), Ok(("", "abc")));
        assert_eq!(span(parser_span)("abcd"), Ok(("d", "abc")));

        let abd = "abd";

        match span(parser_span)(abd).unwrap_err() {
            Err::Error(error) | Err::Failure(error) => assert_eq!(
                error,
                Error {
                    nom_error: Some(NomError {
                        line: 1,
                        column: 3,
                        input: &abd[2..],
                        kind: ErrorKind::Char,
                    }),
                    sprint_error: None,
                }
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn parse_padding0() {
        assert_eq!(padding0(parser)("abc"), Ok(("", "abc")));
        assert_eq!(padding0(parser)(" abc"), Ok(("", "abc")));
        assert_eq!(padding0(parser)("  abc"), Ok(("", "abc")));
        assert_eq!(padding0(parser)("abc "), Ok(("", "abc")));
        assert_eq!(padding0(parser)("abc  "), Ok(("", "abc")));
        assert_eq!(padding0(parser)(" abc "), Ok(("", "abc")));
        assert_eq!(padding0(parser)("  abc  "), Ok(("", "abc")));
    }

    #[test]
    fn parse_brackets1() {
        assert_eq!(brackets1(parser)("(abc)"), Ok(("", "abc")));
        brackets1(parser)("(abc").unwrap_err();
        brackets1(parser)("((abc").unwrap_err();
        brackets1(parser)("((abc)").unwrap_err();
        brackets1(parser)("(((abc))").unwrap_err();
    }
}
