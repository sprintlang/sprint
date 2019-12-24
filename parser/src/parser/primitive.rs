#![allow(unused_parens)]

use super::{builder::definition, context::Context, unify::Unify};
use crate::ast::{
    state::{Effect, State, Transition},
    {Expression, Kind, Observable, Variable},
};
use phf::phf_map;

type Primitive = fn(Vec<Expression>) -> Context<Expression>;

pub static PRIMITIVES: phf::Map<&'static str, Primitive> = phf_map! {
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

pub fn zero() -> Context<'static, Expression<'static>> {
    definition("zero", vec![], Expression::State(State::default()).into()).unwrap()
}

pub fn one() -> Context<'static, Expression<'static>> {
    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    definition("one", vec![], Expression::State(State::default()).into()).unwrap()
}

pub fn give() -> Context<'static, Expression<'static>> {
    let next = Expression::Variable(Variable::new("next", Kind::State.into()));

    let mut transition = Transition::default();
    transition.add_effect(Effect::Flip).set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition("give", vec!["next"], Expression::State(state).into()).unwrap()
}

pub fn and() -> Context<'static, Expression<'static>> {
    let left = Expression::Variable(Variable::new("left", Kind::State.into()));
    let right = Expression::Variable(Variable::new("right", Kind::State.into()));

    let mut transition = Transition::default();
    transition.add_effect(Effect::Spawn(right)).set_next(left);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        "and",
        vec!["left", "right"],
        Expression::State(state).into(),
    )
    .unwrap()
}

pub fn or() -> Context<'static, Expression<'static>> {
    let left = Expression::Variable(Variable::new("left", Kind::State.into()));
    let right = Expression::Variable(Variable::new("right", Kind::State.into()));

    let mut left_transition = Transition::default();
    left_transition
        .add_condition(Observable::IsParty.into())
        .set_next(left);

    let mut right_transition = Transition::default();
    right_transition
        .add_condition(Observable::IsParty.into())
        .set_next(right);

    let mut state = State::default();
    state
        .add_transition(left_transition)
        .add_transition(right_transition);

    definition("or", vec!["left", "right"], Expression::State(state).into()).unwrap()
}

pub fn scale() -> Context<'static, Expression<'static>> {
    let scalar = Expression::Variable(Variable::new(
        "scalar",
        Kind::Observable(Kind::Word.into()).into(),
    ));
    let next = Expression::Variable(Variable::new("next", Kind::State.into()));

    let mut transition = Transition::default();
    transition.add_effect(Effect::Scale(scalar)).set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        "scale",
        vec!["scalar", "next"],
        Expression::State(state).into(),
    )
    .unwrap()
}

pub fn anytime() -> Context<'static, Expression<'static>> {
    let next = Expression::Variable(Variable::new("next", Kind::State.into()));

    let mut transition = Transition::default();
    transition
        .add_condition(Observable::IsParty.into())
        .set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition("anytime", vec!["next"], Expression::State(state).into()).unwrap()
}

pub fn konst(arguments: Vec<Expression>) -> Context<Expression> {
    let value = arguments!(arguments, Kind::default());
    Expression::Observable(value.into()).into()
}

fn argument<'a>(
    arguments: &mut impl Iterator<Item = Expression<'a>>,
    kind: Kind,
) -> Expression<'a> {
    let argument = arguments
        .next()
        .expect("invalid number of arguments in primitive application");

    argument
        .kind()
        .unify(kind.into())
        .expect("invalid argument kind in primitive application");

    argument
}
