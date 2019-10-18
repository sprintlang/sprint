use super::{
    combinator::{brackets, padding},
    IResult, Span,
};
use crate::ast::{
    expression::kind::Observable,
    state::{Effect, State, Transition},
};
use nom::{branch::alt, bytes::complete::tag, sequence::tuple};
use std::{cell::RefCell, rc::Rc};

pub fn contract(input: Span) -> IResult<Span, State> {
    padding(alt((brackets(contract), zero, one, give, and, or, anytime)))(input)
}

pub fn zero(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("zero")(input)?;
    Ok((input, State::default()))
}

pub fn one(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("one")(input)?;

    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    Ok((input, state))
}

pub fn give(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("give")(input)?;
    let (input, mut state) = contract(input)?;

    for transition in state.transitions_mut() {
        transition.add_effect(Effect::Flip);
    }

    Ok((input, state))
}

pub fn and(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("and")(input)?;
    let (input, (_left, _right)) = tuple((contract, contract))(input)?;

    // TODO: implement
    Ok((input, State::default()))
}

pub fn or(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("or")(input)?;
    let (input, (left, right)) = tuple((contract, contract))(input)?;

    let is_holder = Rc::new(Observable::IsHolder);

    let mut left_transition = Transition::default();
    left_transition
        .add_condition(is_holder.clone())
        .set_next(Rc::new(left.into()));

    let mut right_transition = Transition::default();
    right_transition
        .add_condition(is_holder.clone())
        .set_next(Rc::new(right.into()));

    let mut state = State::default();
    state
        .add_transition(left_transition)
        .add_transition(right_transition);

    Ok((input, state))
}

pub fn anytime(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("anytime")(input)?;
    let (input, next) = contract(input)?;

    let mut transition = Transition::default();
    transition
        .add_condition(Rc::new(Observable::IsHolder))
        .set_next(Rc::new(RefCell::new(next)));

    let mut state = State::default();
    state.add_transition(transition);

    Ok((input, state))
}
