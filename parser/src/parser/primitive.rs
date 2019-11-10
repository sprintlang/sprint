#![allow(unused_parens)]

use super::{context::Context, unify::Unify};
use crate::ast::{
    state::{Effect, State, Transition},
    {Expression, Kind, Observable},
};
use phf::phf_map;

pub static PRIMITIVES: phf::Map<&'static str, Primitive> = phf_map! {
    "zero" => Primitive::Zero,
    "one" => Primitive::One,
    "give" => Primitive::Give,
    "and" => Primitive::And,
    "or" => Primitive::Or,
    "scale" => Primitive::Scale,
    "anytime" => Primitive::Anytime,
    "konst" => Primitive::Konst,
};

#[derive(Clone)]
pub enum Primitive {
    Zero,
    One,
    Give,
    And,
    Or,
    Scale,
    Anytime,
    Konst,
}

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

impl Primitive {
    pub fn build(&self, arguments: Vec<Expression>) -> Context<'static, Expression> {
        match self {
            Self::Zero => {
                arguments!(arguments);
                Expression::State(State::default()).into()
            }

            Self::One => {
                arguments!(arguments);

                let mut transition = Transition::default();
                transition.add_effect(Effect::Withdraw);

                let mut state = State::default();
                state.add_transition(transition);

                Expression::State(state).into()
            }

            Self::Give => {
                let next = arguments!(arguments, Kind::State);

                let mut transition = Transition::default();
                transition.add_effect(Effect::Flip).set_next(next);

                let mut state = State::default();
                state.add_transition(transition);

                Expression::State(state).into()
            }

            Self::And => {
                let (left, right) = arguments!(arguments, Kind::State, Kind::State);

                let mut transition = Transition::default();
                transition.add_effect(Effect::Spawn(right)).set_next(left);

                let mut state = State::default();
                state.add_transition(transition);

                Expression::State(state).into()
            }

            Self::Or => {
                let (left, right) = arguments!(arguments, Kind::State, Kind::State);

                let mut left_transition = Transition::default();
                left_transition
                    .add_condition(Observable::IsHolder.into())
                    .set_next(left);

                let mut right_transition = Transition::default();
                right_transition
                    .add_condition(Observable::IsHolder.into())
                    .set_next(right);

                let mut state = State::default();
                state
                    .add_transition(left_transition)
                    .add_transition(right_transition);

                Expression::State(state).into()
            }

            Self::Scale => {
                let (scalar, next) =
                    arguments!(arguments, Kind::Observable(Kind::Word.into()), Kind::State);

                let mut transition = Transition::default();
                transition.add_effect(Effect::Scale(scalar)).set_next(next);

                let mut state = State::default();
                state.add_transition(transition);

                Expression::State(state).into()
            }

            Self::Anytime => {
                let next = arguments!(arguments, Kind::State);

                let mut transition = Transition::default();
                transition
                    .add_condition(Observable::IsHolder.into())
                    .set_next(next);

                let mut state = State::default();
                state.add_transition(transition);

                Expression::State(state).into()
            }

            Self::Konst => {
                let value = arguments!(arguments, Kind::default());
                Expression::Observable(value.into()).into()
            }
        }
    }
}

fn argument(arguments: &mut impl Iterator<Item = Expression>, kind: Kind) -> Expression {
    let argument = arguments
        .next()
        .expect("invalid number of arguments in primitive application");

    argument
        .kind()
        .unify(kind.into())
        .expect("invalid argument kind in primitive application");

    argument
}
