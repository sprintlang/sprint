mod builder;
mod combinator;
mod context;
mod error;
mod primitive;
mod program;
mod unify;

use self::{combinator::span, error::Error, program::program};
use crate::ast::Definition;
use nom::{
    combinator::{all_consuming, complete},
    Err,
};
use nom_locate::LocatedSpan;
use std::{collections::HashMap, rc::Rc, result};

type Span<'a> = LocatedSpan<&'a str>;

type Result<'a, T> = result::Result<T, Err<Error<'a>>>;

type IResult<'a, I, O> = nom::IResult<I, O, Error<'a>>;

pub fn contract<'a>(input: &'a str) -> result::Result<HashMap<&'a str, Rc<Definition<'a>>>, Error> {
    match span(all_consuming(complete(program)))(input) {
        Ok((_, context)) => Ok(context.definitions),
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
