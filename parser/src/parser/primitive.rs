#![allow(unused_parens)]

use super::{
    builder::definition, context::Context, error::SprintError, unify::Unify, Error, Result, Span,
};
use crate::ast::{
    state::{Effect, State, Transition},
    {Expression, ExpressionType, Kind, Observable, Variable},
};
use nom::Err;
use phf::phf_map;

type Primitive = fn(Vec<Expression>) -> Result<Context<Expression>>;

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

pub fn zero() -> Result<'static, Context<'static, Expression<'static>>> {
    definition(
        Span::new("zero"),
        vec![],
        Expression::new(ExpressionType::State(State::default()), None).into(),
    )
}

pub fn one() -> Result<'static, Context<'static, Expression<'static>>> {
    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("one"),
        vec![],
        Expression::new(ExpressionType::from(state), None).into(),
    )
}

pub fn give() -> Result<'static, Context<'static, Expression<'static>>> {
    let next = Expression::new(
        ExpressionType::from(Variable::new("next", Kind::State.into(), None)),
        None,
    );

    let mut transition = Transition::default();
    transition.add_effect(Effect::Flip).set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("give"),
        vec![Span::new("next")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
}

pub fn and() -> Result<'static, Context<'static, Expression<'static>>> {
    let left = Expression::new(
        ExpressionType::from(Variable::new("left", Kind::State.into(), None)),
        None,
    );

    let right = Expression::new(
        ExpressionType::from(Variable::new("right", Kind::State.into(), None)),
        None,
    );

    let mut transition = Transition::default();
    transition.add_effect(Effect::Spawn(right)).set_next(left);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("and"),
        vec![Span::new("left"), Span::new("right")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
}

pub fn or() -> Result<'static, Context<'static, Expression<'static>>> {
    let left = Expression::new(
        ExpressionType::from(Variable::new("left", Kind::State.into(), None)),
        None,
    );
    let right = Expression::new(
        ExpressionType::from(Variable::new("right", Kind::State.into(), None)),
        None,
    );

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
    definition(
        Span::new("or"),
        vec![Span::new("left"), Span::new("right")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
}

pub fn scale() -> Result<'static, Context<'static, Expression<'static>>> {
    let scalar = Expression::new(
        ExpressionType::from(Variable::new(
            "scalar",
            Kind::Observable(Kind::Word.into()).into(),
            None,
        )),
        None,
    );

    let next = Expression::new(
        ExpressionType::from(Variable::new("next", Kind::State.into(), None)),
        None,
    );

    let mut transition = Transition::default();
    transition.add_effect(Effect::Scale(scalar)).set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("scale"),
        vec![Span::new("scalar"), Span::new("next")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
}

pub fn anytime() -> Result<'static, Context<'static, Expression<'static>>> {
    let next = Expression::new(
        ExpressionType::from(Variable::new("next", Kind::State.into(), None)),
        None,
    );

    let mut transition = Transition::default();
    transition
        .add_condition(Expression::new(Observable::IsParty.into(), next.span))
        .set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("anytime"),
        vec![Span::new("next")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
}

pub fn konst(arguments: Vec<Expression>) -> Result<'_, Context<Expression>> {
    let value = arguments!(arguments, Kind::default())?;
    let span = value.span;

    Ok(Expression::new(ExpressionType::Observable(value.into()), span).into())
}

fn argument<'a>(
    arguments: &mut impl Iterator<Item = Expression<'a>>,
    kind: Kind,
) -> Result<'a, Expression<'a>> {
    let argument = match arguments.next() {
        Some(argument) => argument,
        None => {
            return Err(Err::Error(Error::from_sprint_error(
                SprintError::InvalidNumberArgsError,
            )));
        }
    };
    argument
        .expression
        .kind()
        .unify(kind.into())
        .map_err(Err::Error)?;

    Ok(argument)
}
