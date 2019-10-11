pub mod contract;
pub mod error;

mod combinator;

use self::{combinator::span, error::Error};
use crate::ast::contract::Contract;
use nom::{combinator::all_consuming, IResult};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

pub fn contract(input: &str) -> Result<Contract, String> {
    fn all_consuming_contract(input: Span) -> IResult<Span, Contract, Error> {
        all_consuming(self::contract::contract)(input)
    }

    match span(all_consuming_contract)(input) {
        Ok((_, contract)) => Ok(contract),
        Err(nom::Err::Error(_)) | Err(nom::Err::Failure(_)) => Err("Error".into()),
        Err(nom::Err::Incomplete(_)) => Err(String::from("Incomplete input")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contract() {
        assert_eq!(contract("zero"), Ok(Contract::Zero));
    }
}
