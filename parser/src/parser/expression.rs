use super::{combinator::padding, IResult, Span};
use crate::ast::expression::{Expression, Observable};
use nom::{branch::alt, bytes::complete::tag, character::complete::digit1};

pub fn expression(input: Span) -> IResult<Span, Expression> {
    padding(alt((word, observable_konst)))(input)
}

pub fn literal(input: Span) -> IResult<Span, Expression> {
    padding(word)(input)
}

// TODO: require definitions of observables, with types.
pub fn word(input: Span) -> IResult<Span, Expression> {
    let (input, number) = digit1(input)?;
    let number = number.fragment.parse::<u64>().unwrap();
    Ok((input, Expression::Word(number)))
}

pub fn observable_konst(input: Span) -> IResult<Span, Expression> {
    let (input, _) = tag("konst")(input)?;
    let (input, expr) = literal(input)?;
    Ok((
        input,
        Expression::Observable(Observable::Konst(Box::new(expr))),
    ))
}

#[cfg(test)]
mod tests {
    use super::super::combinator::span;
    use super::*;
    use nom::combinator::all_consuming;

    fn parse_expression_ok(input: &str, expected: (&str, Expression)) {
        assert_eq!(span(expression)(input), Ok(expected));
    }

    fn parse_expression_err(input: &str) {
        assert!(span(all_consuming(expression))(input).is_err());
    }

    #[test]
    fn parse_word() {
        parse_expression_ok("123", ("", Expression::Word(123)));

        parse_expression_err("-5");
    }

    #[test]
    fn parse_observable_konst() {
        parse_expression_ok(
            "konst 123",
            (
                "",
                Expression::Observable(Observable::Konst(Box::new(Expression::Word(123)))),
            ),
        );

        parse_expression_err("konst");
        parse_expression_err("konst -5");
        parse_expression_err("konst konst 123");
    }
}
