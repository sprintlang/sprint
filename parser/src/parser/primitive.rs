#![allow(unused_parens)]

use super::{
    context::Context,
    error::{CombinedError, SprintError},
    unify::Unify,
    Result, Span,
};
use crate::ast::{
    state::{Effect, State, Transition},
    {Expression, ExpressionType, Kind, Observable},
};
use nom::{error::ErrorKind, Err};
use phf::phf_map;

type Primitive = fn(Vec<Expression>) -> Result<'_, Context<Expression>>;

pub static PRIMITIVES: phf::Map<&'static str, Primitive> = phf_map! {
    "zero" => zero,
    "one" => one,
    "give" => give,
    "and" => and,
    "or" => or,
    "scale" => scale,
    "anytime" => anytime,
    "konst" => konst,
};

macro_rules! arguments {
    ($arguments:expr $(, $kind:expr)*) => {
        {
            let mut arguments = $arguments.into_iter();
            let expressions = ($(argument(&mut arguments, $kind)),*);

            assert!(
                arguments.next().is_none(),
                "invalid number of arguments in primitive application"
            );

            expressions
        }
    };
}

pub fn zero(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    arguments!(arguments);
    Ok(Expression::new(ExpressionType::State(State::default()), Span::new("")).into())
}

pub fn one(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    arguments!(arguments);

    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    Ok(Expression::new(ExpressionType::State(state), Span::new("")).into())
}

pub fn give(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let next = arguments!(arguments, Kind::State)?;
    let span = next.span;

    let mut transition = Transition::default();
    transition.add_effect(Effect::Flip).set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    Ok(Expression::new(ExpressionType::State(state), span).into())
}

pub fn and(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let (left, right) = arguments!(arguments, Kind::State, Kind::State);
    let left = left?;
    let right = right?;
    let left_span = left.span;

    let mut transition = Transition::default();
    transition.add_effect(Effect::Spawn(right)).set_next(left);

    let mut state = State::default();
    state.add_transition(transition);

    // TODO: different span?
    Ok(Expression::new(ExpressionType::State(state), left_span).into())
}

pub fn or(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let (left, right) = arguments!(arguments, Kind::State, Kind::State);
    let left = left?;
    let right = right?;
    let left_span = left.span;

    let mut left_transition = Transition::default();
    left_transition
        .add_condition(Expression::new(Observable::IsParty.into(), left.span))
        .set_next(left);

    let mut right_transition = Transition::default();
    right_transition
        .add_condition(Expression::new(Observable::IsParty.into(), right.span))
        .set_next(right);

    let mut state = State::default();
    state
        .add_transition(left_transition)
        .add_transition(right_transition);

    // TODO: different span?
    Ok(Expression::new(ExpressionType::State(state), left_span).into())
}

pub fn scale(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let (scalar, next) = arguments!(arguments, Kind::Observable(Kind::Word.into()), Kind::State);
    let scalar = scalar?;
    let next = next?;
    let next_span = next.span;

    let mut transition = Transition::default();
    transition.add_effect(Effect::Scale(scalar)).set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    // TODO: different span?
    Ok(Expression::new(ExpressionType::State(state), next_span).into())
}

pub fn anytime(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let next = arguments!(arguments, Kind::State)?;
    let next_span = next.span;

    let mut transition = Transition::default();
    transition
        .add_condition(Expression::new(Observable::IsParty.into(), next.span))
        .set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    // TODO: different span?
    Ok(Expression::new(ExpressionType::State(state), next_span).into())
}

pub fn konst(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let value = arguments!(arguments, Kind::default())?;
    let value_span = value.span;
    // TODO: different span?
    Ok(Expression::new(ExpressionType::Observable(value.into()), value_span).into())
}

fn argument<'a>(
    arguments: &mut impl Iterator<Item = Expression<'a>>,
    kind: Kind,
) -> Result<'a, Expression<'a>> {
    let argument = match arguments.next() {
        Some(argument) => argument,
        None => {
            return Err(Err::Error(CombinedError::from_sprint_error(
                SprintError::InvalidNumberArgsError,
            )));
        }
    };
    argument
        .expression
        .kind()
        .unify(kind.into())
        .map_err(|err| Err::Error(err))?;
    // .map_err(|sprint_error| {
    //     Err::Error(CombinedError::from_sprint_error_and_error_kind(
    //         argument.span,
    //         ErrorKind::Tag,
    //         sprint_error,
    //     ))
    // })?;

    Ok(argument)
}
