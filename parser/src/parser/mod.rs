mod builder;
mod combinator;
mod context;
mod date;
mod error;
mod primitive;
mod program;
mod unify;

use self::{combinator::span, error::Error, program::program};
use crate::ast::Definitions;
use nom::{
    combinator::{all_consuming, complete},
    Err,
};
use nom_locate::LocatedSpan;
use std::result;

pub type Span<'a> = LocatedSpan<&'a str>;

type Result<'a, T> = result::Result<T, Err<Error<'a>>>;

type IResult<'a, I, O> = nom::IResult<I, O, Error<'a>>;

pub fn contract<'a>(input: &'a str) -> result::Result<Definitions<'a>, Error> {
    match span(all_consuming(complete(program)))(input) {
        Ok((_, context)) => {
            let variables = &context.variables;
            Ok(context
                .definitions
                .into_iter()
                .map(|(_, d)| d)
                // TODO: giving "main" an initial count of 1 would be nicer.
                .filter(|d| d.variable.name == "main" || variables.count(&d.variable) > 1)
                .collect())
        }
        Err(nom::Err::Error(error)) | Err(nom::Err::Failure(error)) => Err(error),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_contract() {
        assert!(contract("main = zero").is_ok());
    }
}
