use super::{combinator::padding, IResult, Span};
use crate::ast::expression::{Expression, Observable};
use nom::character::complete::digit1;

pub fn observable(input: Span) -> IResult<Span, Observable> {
    padding(literal)(input)
}

// TODO: require definitions of observables, with types.
pub fn literal(input: Span) -> IResult<Span, Observable> {
    let (input, number) = digit1(input)?;
    let number = number.fragment.parse::<u64>().unwrap();
    Ok((input, Observable::Konst(Box::new(Expression::Word(number)))))
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
        );

        parse_observable_ok("0", ("", Observable::Konst(Box::new(Expression::Word(0)))));
    }
}
