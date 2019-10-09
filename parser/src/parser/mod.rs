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
    use nom::IResult;
    use std::fmt::Debug;

    pub fn parse<T: Debug + PartialEq>(
        input: &str,
        parser: impl Fn(&str) -> IResult<&str, T>,
        expected: (&str, T),
    ) {
        match parser(input) {
            Ok(actual) => assert_eq!(
                actual, expected,
                "Expected {:?} parsing \"{}\", but got {:?}",
                expected, input, actual
            ),
            Err(error) => panic!(
                "Expected {:?} parsing \"{}\", but got error {:?}",
                expected, input, error
            ),
        };
    }

    pub fn parse_invalid<T: Debug>(input: &str, parser: impl Fn(&str) -> IResult<&str, T>) {
        if let Ok(output) = parser(input) {
            panic!("Expected error parsing \"{}\", but got {:?}", input, output);
        }
    }

    #[test]
    fn parse_contract() {
        assert_eq!(contract("zero"), Ok(ast::Contract::Zero));
    }
}
