use crate::ast::Contract;
use nom::{branch::alt, bytes::complete::tag, IResult};

pub fn contract(input: &str) -> IResult<&str, Contract> {
    let parser = alt((zero, one));
    parser(input)
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
    use super::super::tests::{parse, parse_invalid};
    use super::*;

    #[test]
    fn parse_zero() {
        parse("zero", contract, ("", Contract::Zero));
    }

    #[test]
    fn parse_one() {
        parse("one", contract, ("", Contract::One));
    }

    #[test]
    fn parse_invalid_two() {
        parse_invalid("two", contract);
    }
}
