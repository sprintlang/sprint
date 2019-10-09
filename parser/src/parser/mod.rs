pub mod contract;

mod combinator;

use crate::ast;
use nom::{
    combinator::all_consuming,
    error::{convert_error, ParseError, VerboseError},
};

pub fn contract(input: &str) -> Result<ast::Contract, String> {
    match all_consuming(self::contract::contract)(input) {
        Ok((_, contract)) => Ok(contract),
        Err(nom::Err::Error((input, kind))) | Err(nom::Err::Failure((input, kind))) => {
            let error = VerboseError::from_error_kind(input, kind);
            Err(convert_error(input, error))
        }
        Err(nom::Err::Incomplete(_)) => Err(String::from("Incomplete input")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contract() {
        assert_eq!(contract("zero"), Ok(ast::Contract::Zero));
    }
}
