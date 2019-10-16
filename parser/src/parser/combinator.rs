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

pub fn padding<I, O, E, F>(f: F) -> impl Fn(I) -> nom::IResult<I, O, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
    F: Fn(I) -> nom::IResult<I, O, E>,
{
    delimited(multispace0, f, multispace0)
}

pub fn brackets<I, O, E, F>(f: F) -> impl Fn(I) -> nom::IResult<I, O, E>
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
    use super::super::{error::Error, Span};
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
                    line: 1,
                    column: 3,
                    input: &abd[2..],
                    kind: ErrorKind::Char,
                }
            ),
            _ => unreachable!(),
        }
    }

    #[test]
    fn parse_padding() {
        assert_eq!(padding(parser)("abc"), Ok(("", "abc")));
        assert_eq!(padding(parser)(" abc"), Ok(("", "abc")));
        assert_eq!(padding(parser)("  abc"), Ok(("", "abc")));
        assert_eq!(padding(parser)("abc "), Ok(("", "abc")));
        assert_eq!(padding(parser)("abc  "), Ok(("", "abc")));
        assert_eq!(padding(parser)(" abc "), Ok(("", "abc")));
        assert_eq!(padding(parser)("  abc  "), Ok(("", "abc")));
    }

    #[test]
    fn parse_brackets() {
        assert_eq!(brackets(parser)("(abc)"), Ok(("", "abc")));
        assert_eq!(brackets(parser)("((abc))"), Ok(("", "abc")));
        assert_eq!(brackets(parser)("(((abc)))"), Ok(("", "abc")));
        assert_eq!(brackets(parser)("(abc))"), Ok((")", "abc")));
        brackets(parser)("(abc").unwrap_err();
        brackets(parser)("((abc").unwrap_err();
        brackets(parser)("((abc)").unwrap_err();
        brackets(parser)("(((abc))").unwrap_err();
    }
}
