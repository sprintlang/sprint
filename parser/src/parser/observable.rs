use super::{combinator::padding, IResult, Span};
use crate::ast::expression::{Expression, Observable};
use nom::bytes::complete::tag;

pub fn observable(input: Span) -> IResult<Span, Observable> {
    padding(literal)(input)
}

// TODO: require definitions of observables, with types.
pub fn literal(input: Span) -> IResult<Span, Observable> {
    let (input, _) = tag("123")(input)?;
    Ok((input, Observable::Konst(Box::new(Expression::Word(123)))))
}

#[cfg(test)]
mod tests {
    use super::super::combinator::span;
    use super::*;

    fn parse_observable_ok(input: &str, expected: (&str, Observable)) {
        assert_eq!(span(observable)(input), Ok(expected));
    }

    #[test]
    fn parse_number_literal() {
        parse_observable_ok(
            "123",
            ("", Observable::Konst(Box::new(Expression::Word(123)))),
        )
    }
}
