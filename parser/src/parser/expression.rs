use super::{combinator::padding, IResult, Span};
use crate::ast::expression::{Expression, Observable};
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1};

pub fn expression(input: Span) -> IResult<Span, Expression> {
    padding(alt((literal, konst)))(input)
}

// TODO: require definitions of observables, with types.
pub fn literal(input: Span) -> IResult<Span, Expression> {
    let (input, number) = digit1(input)?;
    let number = number.fragment.parse::<u64>().unwrap();
    Ok((input, Expression::Word(number)))
}

pub fn konst(input: Span) -> IResult<Span, Expression> {
    let (input, _) = tag("konst")(input)?;
    let (input, next) = expression(input)?;
    Ok((
        input,
        Expression::Observable(Observable::Konst(Box::new(next))),
    ))
}

#[cfg(test)]
mod tests {
    use super::super::combinator::span;
    use super::*;

    fn parse_expression_ok(input: &str, expected: (&str, Expression)) {
        assert_eq!(span(expression)(input), Ok(expected));
    }

    #[test]
    fn parse_number_literal() {
        parse_expression_ok("123", ("", Expression::Word(123)));
    }

    #[test]
    fn parse_konst() {
        parse_expression_ok(
            "konst 123",
            (
                "",
                Expression::Observable(Observable::Konst(Box::new(Expression::Word(123)))),
            ),
        );
    }
}
