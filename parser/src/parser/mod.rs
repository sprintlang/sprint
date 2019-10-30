pub mod contract;
pub mod error;
pub mod observable;

mod combinator;

use self::{combinator::span, error::Error};
use crate::ast::state::State;
use nom::combinator::{all_consuming, complete};
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;
type IResult<'a, I, O> = nom::IResult<I, O, Error<'a>>;

pub fn contract(input: &str) -> Result<State, Error> {
    match span(all_consuming(complete(self::contract::contract)))(input) {
        Ok((_, state)) => Ok(state),
        Err(nom::Err::Error(error)) | Err(nom::Err::Failure(error)) => Err(error),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contract() {
        // assert_eq!(contract("zero"), Ok(Contract::Zero));
        assert!(contract("two").is_err());
    }
}
