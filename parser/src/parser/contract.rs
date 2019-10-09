use super::combinator::{brackets, padding};
use crate::ast::contract::Contract;
use nom::{branch::alt, bytes::complete::tag, IResult};

pub fn contract(input: &str) -> IResult<&str, Contract> {
    padding(alt((brackets(contract), zero, one)))(input)
}

pub fn zero(input: &str) -> IResult<&str, Contract> {
    let (input, _) = tag("zero")(input)?;
    Ok((input, Contract::Zero))
}

pub fn one(input: &str) -> IResult<&str, Contract> {
    let (input, _) = tag("one")(input)?;
    Ok((input, Contract::One))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contract_with_padding_and_brackets() {
        assert_eq!(contract(" (zero) "), Ok(("", Contract::Zero)));
        assert_eq!(contract("( zero )"), Ok(("", Contract::Zero)));
        assert_eq!(contract(" ( zero ) "), Ok(("", Contract::Zero)));
        assert_eq!(contract(" ( (zero) ) "), Ok(("", Contract::Zero)));
        assert_eq!(contract(" ( (zero))"), Ok(("", Contract::Zero)));
    }

    #[test]
    fn parse_zero() {
        assert_eq!(contract("zero"), Ok(("", Contract::Zero)));
    }

    #[test]
    fn parse_one() {
        assert_eq!(contract("one"), Ok(("", Contract::One)));
    }

    #[test]
    fn parse_two() {
        contract("two").unwrap_err();
    }
}
