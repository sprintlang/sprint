use super::ast::Contract;
use nom::{branch::alt, bytes::complete::tag, combinator::all_consuming, IResult};

pub fn parse(input: &str) -> Result<Contract, ()> {
    let parser = contract(input);

    match parser {
        Ok((_, node)) => Ok(node),
        Err(_) => Err(()),
    }
}

pub fn contract(input: &str) -> IResult<&str, Contract> {
    let parser = all_consuming(alt((zero, one)));
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
    use super::*;

    fn ast_parsed(input: &str, expected: Contract) {
        match parse(input) {
            Ok(actual) => assert_eq!(
                actual, expected,
                "Tried to parse \"{}\", but got {:?}.",
                input, actual
            ),
            Err(()) => panic!("Error parsing \"{}\".", input),
        };
    }

    fn ast_not_parsed(input: &str) {
        if let Ok(contract) = parse(input) {
            panic!(
                "\"{}\" should not have parsed, but got {:?}.",
                input, contract
            );
        }
    }

    #[test]
    fn zero_parsed() {
        ast_parsed("zero", Contract::Zero);
    }

    #[test]
    fn one_parsed() {
        ast_parsed("one", Contract::One);
    }

    #[test]
    fn invalid_zero_one_combination_not_parsed() {
        ast_not_parsed("zero zero");
        ast_not_parsed("one one");
        ast_not_parsed("zero one");
        ast_not_parsed("one zero");
    }

    #[test]
    fn gibberish_not_parsed() {
        ast_not_parsed("abcd");
    }
}
