use super::{IResult, Span};
use crate::ast::{Date, Expression, ExpressionType};
use nom::{bytes::complete::tag, character::complete::digit1, combinator::opt};

pub fn date(input: Span) -> IResult<Span, Expression> {
    let span = input;

    // Parse date in ISO 8601 format - a year must be specified. Otherwise the
    // default is 0 e.g. no month or day specified defaults to 1st January.
    let (input, year) = digits(input)?;
    let (input, _) = tag("-")(input)?;
    // Try and match digits: if none present, set month and/or day to 1.
    let (input, month) = match opt(digits)(input)? {
        (input, Some(month)) => (input, month),
        (input, None) => (input, 1),
    };

    let (input, _) = tag("-")(input)?;
    let (input, day) = match opt(digits)(input)? {
        (input, Some(day)) => (input, day),
        (input, None) => (input, 1),
    };

    // Parse time in ISO 8601 format. The default is 0 e.g. no hour, minute or
    // second specified defaults to 00:00:00.
    let (input, _) = tag("T")(input)?;
    // Try and match digits: if none present set hour, minute or second to 1.
    let (input, hour) = match opt(digits)(input)? {
        (input, Some(hour)) => (input, hour),
        (input, None) => (input, 0),
    };

    let (input, _) = tag(":")(input)?;
    let (input, minute) = match opt(digits)(input)? {
        (input, Some(month)) => (input, month),
        (input, None) => (input, 0),
    };

    let (input, _) = tag(":")(input)?;
    let (input, second) = match opt(digits)(input)? {
        (input, Some(second)) => (input, second),
        (input, None) => (input, 0),
    };
    let (input, _) = tag("Z")(input)?;

    // Year is represented as an i32 (compared to u32).
    Ok((
        input,
        Expression::new(
            ExpressionType::Date(Date::Date(year, month, day, hour, minute, second)),
            Some(span),
        ),
    ))
}

fn digits(input: Span) -> IResult<Span, u64> {
    let (input, number) = digit1(input)?;
    let number = number.fragment.parse::<u64>().unwrap();

    Ok((input, number))
}
