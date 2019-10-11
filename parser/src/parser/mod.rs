pub mod contract;
pub mod error;

mod combinator;

use self::{combinator::span, error::Error};
use crate::ast::contract::Contract;
use nom::{
    combinator::{all_consuming, complete},
    IResult,
};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub fn contract(input: &str) -> Result<Contract, Error> {
    fn contract_(input: Span) -> IResult<Span, Contract, Error> {
        all_consuming(complete(self::contract::contract))(input)
    }

    match span(contract_)(input) {
        Ok((_, contract)) => Ok(contract),
        Err(nom::Err::Error(error)) | Err(nom::Err::Failure(error)) => Err(error),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contract() {
        assert_eq!(contract("zero"), Ok(Contract::Zero));
        assert!(contract("two").is_err());
    }
}
