use nom::{branch::alt, bytes::complete::tag, combinator::all_consuming, IResult};

// TODO: implement a real AST (this is temporary)
#[derive(Debug, PartialEq)]
pub enum AST {
    Zero,
    One,
}

fn zero(code: &str) -> IResult<&str, AST> {
    let (code, _) = tag("zero")(code)?;
    Ok((code, AST::Zero))
}

fn one(code: &str) -> IResult<&str, AST> {
    let (code, _) = tag("one")(code)?;
    Ok((code, AST::One))
}

pub fn parse(code: &str) -> Result<AST, ()> {
    let parser = all_consuming(alt((zero, one)));

    match parser(code) {
        Ok((_, ast)) => Ok(ast),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ast_parsed(code: &str, ast_expected: AST) {
        match parse(code) {
            Ok(ast) => assert_eq!(
                ast, ast_expected,
                "Tried to parse \"{}\", but got {:?}.",
                code, ast
            ),
            Err(()) => panic!("Error parsing \"{}\".", code),
        };
    }

    fn ast_not_parsed(code: &str) {
        if let Ok(ast) = parse(code) {
            panic!("\"{}\" should not have parsed, but got {:?}.", code, ast);
        }
    }

    #[test]
    fn zero_parsed() {
        ast_parsed("zero", AST::Zero);
    }

    #[test]
    fn one_parsed() {
        ast_parsed("one", AST::One);
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
