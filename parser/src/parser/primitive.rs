#![allow(unused_parens)]

use super::{
    builder::definition, context::Context, error::SprintError, unify::Unify, Error, Result, Span,
};
use crate::ast::{
    state::{Effect, State, Transition},
    {Class, Comparable, Date, Expression, ExpressionType, Kind, Observable, Variable},
};
use nom::Err;
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
    definition(
        Span::new("zero"),
        vec![],
        Expression::new(ExpressionType::from(State::default()), None).into(),
    )
    .unwrap()
}

pub fn one() -> Context<'static, Expression<'static>> {
    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("one"),
        vec![],
        Expression::new(ExpressionType::from(state), None).into(),
    )
    .unwrap()
}

pub fn give() -> Context<'static, Expression<'static>> {
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
    .unwrap()
}

pub fn and() -> Context<'static, Expression<'static>> {
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
    .unwrap()
}

pub fn or() -> Context<'static, Expression<'static>> {
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
    .unwrap()
}

pub fn before() -> Context<'static, Expression<'static>> {
    let now = Expression::new(ExpressionType::Date(Date::Now), None);

    let date = Expression::new(
        ExpressionType::from(Variable::new("date", Kind::Date.into(), None)),
        None,
    );

    let next = Expression::new(
        ExpressionType::from(Variable::new("next", Kind::State.into(), None)),
        None,
    );

    let mut transition = Transition::default();
    transition
        .add_condition(Expression::new(
            ExpressionType::Class(Class::Comparable(Comparable::Less(now.into(), date.into()))),
            None,
        ))
        .set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("before"),
        vec![Span::new("date"), Span::new("next")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
    .unwrap()
}

pub fn after() -> Context<'static, Expression<'static>> {
    let now = Expression::new(ExpressionType::Date(Date::Now), None);

    let date = Expression::new(
        ExpressionType::from(Variable::new("date", Kind::Date.into(), None)),
        None,
    );

    let next = Expression::new(
        ExpressionType::from(Variable::new("next", Kind::State.into(), None)),
        None,
    );

    let mut transition = Transition::default();
    transition
        .add_condition(Expression::new(
            ExpressionType::Class(Class::Comparable(Comparable::Greater(
                now.into(),
                date.into(),
            ))),
            None,
        ))
        .set_next(next);

    let mut state = State::default();
    state.add_transition(transition);

    definition(
        Span::new("after"),
        vec![Span::new("date"), Span::new("next")],
        Expression::new(ExpressionType::from(state), None).into(),
    )
    .unwrap()
}

pub fn scale() -> Context<'static, Expression<'static>> {
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
    .unwrap()
}

pub fn anytime() -> Context<'static, Expression<'static>> {
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
    .unwrap()
}

pub fn konst(arguments: Vec<Expression>) -> Context<Expression> {
    let value = arguments!(arguments, Kind::default()).unwrap();

    Expression::new(ExpressionType::Observable(value.clone().into()), value.span).into()
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
                None,
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
