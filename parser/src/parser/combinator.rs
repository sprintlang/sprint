use nom::{
    character::complete::char,
    character::complete::multispace0,
    error::ParseError,
    multi::{count, many1_count},
    sequence::delimited,
    AsChar, IResult, InputIter, InputTakeAtPosition, Slice,
};
use std::ops::RangeFrom;

pub fn padding<I, O, E, F>(f: F) -> impl Fn(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
    E: ParseError<I>,
    F: Fn(I) -> IResult<I, O, E>,
{
    delimited(multispace0, f, multispace0)
}

pub fn brackets<I, O, E, F>(f: F) -> impl Fn(I) -> IResult<I, O, E>
where
    I: Slice<RangeFrom<usize>> + InputIter + Clone + PartialEq,
    <I as InputIter>::Item: AsChar,
    E: ParseError<I>,
    F: Fn(I) -> IResult<I, O, E>,
{
    move |input: I| {
        let (input, brackets) = many1_count(char('('))(input)?;
        let (input, output) = f(input)?;
        let (input, _) = count(char(')'), brackets)(input)?;
        Ok((input, output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::bytes::complete::tag;

    fn parser(input: &str) -> IResult<&str, &str> {
        tag("abc")(input)
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
