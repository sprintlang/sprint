pub mod contract;

#[cfg(test)]
mod tests {
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
}
