use super::{
    builder,
    combinator::{brackets1, padding0},
    context::Context,
    date::date,
    IResult, Span,
};
use crate::ast::{Expression, ExpressionType, Kind};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric0, digit1, line_ending, multispace1, space1},
    combinator::{map, map_res, peek, recognize},
    multi::{many0, many1, separated_list},
    sequence::{pair, preceded, separated_pair},
};

pub fn program(input: Span) -> IResult<Span, Context<()>> {
    let separator = alt((line_ending, padding0(tag(";"))));
    let (input, contexts) = separated_list(many1(separator), alt((signature, definition)))(input)?;
    let (input, _) = many0(alt((multispace1, tag(";"))))(input)?;

    Ok((input, builder::program(contexts)?))
}

pub fn signature(input: Span) -> IResult<Span, Context<Expression>> {
    let (input, identifier) = identifier(input)?;
    let (input, _) = padding0(tag("::"))(input)?;
    let (input, kind) = kind(input)?;

    Ok((input, builder::signature(identifier, kind)?))
}

pub fn kind(input: Span) -> IResult<Span, Kind> {
    alt((
        map(
            separated_pair(kind_primitive, padding0(tag("->")), kind),
            |(from, to)| Kind::Abstraction(from.into(), to.into()),
        ),
        kind_primitive,
    ))(input)
}

pub fn kind_primitive(input: Span) -> IResult<Span, Kind> {
    alt((
        brackets1(kind),
        map(tag("Bool"), |_| Kind::Boolean),
        map(preceded(pair(tag("Observable"), separator), kind), |k| {
            Kind::Observable(k.into())
        }),
        map(tag("Contract"), |_| Kind::State),
        map(tag("Word"), |_| Kind::Word),
    ))(input)
}

pub fn definition(input: Span) -> IResult<Span, Context<Expression>> {
    let (input, id) = identifier(input)?;

    let argument = preceded(multispace1, identifier);
    let (input, arguments) = many0(argument)(input)?;

    let (input, _) = padding0(tag("="))(input)?;
    let (input, expression) = expression(input)?;

    Ok((input, builder::definition(id, arguments, expression)?))
}

pub fn expression(input: Span) -> IResult<Span, Context<Expression>> {
    alt((application, term))(input)
}

pub fn application(input: Span) -> IResult<Span, Context<Expression>> {
    let (input, identifier) = identifier(input)?;

    let argument = preceded(separator, term);
    let (input, arguments) = many0(argument)(input)?;
    Ok((input, builder::application(identifier, arguments)?))
}

pub fn term(input: Span) -> IResult<Span, Context<Expression>> {
    alt((
        brackets1(expression),
        map_res(identifier, |identifier| {
            builder::application(identifier, Vec::new())
        }),
        map(tag("True"), |span| {
            Expression::new(ExpressionType::from(true), Some(span)).into()
        }),
        map(tag("False"), |span| {
            Expression::new(ExpressionType::from(false), Some(span)).into()
        }),
        map(date, Context::from),
        map(digit1, |n: Span| {
            Expression::new(
                ExpressionType::from(n.fragment.parse::<u64>().unwrap()),
                Some(n),
            )
            .into()
        }),
    ))(input)
}

pub fn identifier(input: Span) -> IResult<Span, Span> {
    let lowercase1 = take_while1(|c: char| c.is_ascii_lowercase());
    let (input, identifier) = recognize(pair(lowercase1, alphanumeric0))(input)?;

    Ok((input, identifier))
}

fn separator(input: Span) -> IResult<Span, Span> {
    alt((space1, peek(tag("("))))(input)
}
