use super::{
    combinator::{brackets, padding},
    error::Error,
    Span,
};
use crate::ast::contract::Contract;
use nom::{branch::alt, bytes::complete::tag, IResult};

pub fn contract(input: Span) -> IResult<Span, Contract, Error> {
    padding(alt((brackets(contract), zero, one, or, give)))(input)
}

pub fn zero(input: Span) -> IResult<Span, Contract, Error> {
    let (input, _) = tag("zero")(input)?;
    Ok((input, Contract::Zero))
}

pub fn one(input: Span) -> IResult<Span, Contract, Error> {
    let (input, _) = tag("one")(input)?;
    Ok((input, Contract::One))
}

pub fn or(input: Span) -> IResult<Span, Contract, Error> {
    let (input, _) = tag("or")(input)?;
    let (input, first_contract) = contract(input)?;
    let (input, second_contract) = contract(input)?;
    Ok((
        input,
        Contract::Or(Box::new(first_contract), Box::new(second_contract)),
    ))
}

pub fn give(input: Span) -> IResult<Span, Contract, Error> {
    let (input, _) = tag("give")(input)?;
    let (input, contract) = contract(input)?;
    Ok((input, Contract::Give(Box::new(contract))))
}

#[cfg(test)]
mod tests {
    use super::super::combinator::span;
    use super::*;

    fn parse_contract_ok(input: &str, expected: (&str, Contract)) {
        assert_eq!(span(contract)(input), Ok(expected));
    }

    fn parse_contract_err(input: &str) {
        assert!(span(contract)(input).is_err());
    }

    #[test]
    fn parse_contract_with_padding_and_brackets() {
        parse_contract_ok(" (zero) ", ("", Contract::Zero));
        parse_contract_ok("( zero )", ("", Contract::Zero));
        parse_contract_ok(" ( zero ) ", ("", Contract::Zero));
        parse_contract_ok(" ( (zero) ) ", ("", Contract::Zero));
        parse_contract_ok(" ( (zero))", ("", Contract::Zero));
    }

    #[test]
    fn parse_zero() {
        parse_contract_ok("zero", ("", Contract::Zero));
    }

    #[test]
    fn parse_one() {
        parse_contract_ok("one", ("", Contract::One));
    }

    #[test]
    fn parse_two() {
        parse_contract_err("two");
    }

    #[test]
    fn parse_give() {
        parse_contract_ok("give zero", ("", Contract::Give(Box::new(Contract::Zero))));

        parse_contract_ok(
            "give give zero",
            (
                "",
                Contract::Give(Box::new(Contract::Give(Box::new(Contract::Zero)))),
            ),
        );
    }

    #[test]
    fn parse_or() {
        parse_contract_ok(
            "or zero one",
            (
                "",
                Contract::Or(Box::new(Contract::Zero), Box::new(Contract::One)),
            ),
        );
    }
}
