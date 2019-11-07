pub mod state;

mod class;

pub use self::class::{Class, Comparable, Equatable, Negatable, Numerable};

use state::State;
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    Abstraction(Rc<Expression>, Rc<Expression>),
    Application(Rc<Expression>, Rc<Expression>),
    Argument(Kind),
    Boolean(bool),
    Class(Class),
    Observable(Observable),
    State(State),
    Word(u64),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Kind {
    Abstraction(Box<Kind>, Box<Kind>),
    Boolean,
    Observable(Box<Kind>),
    State,
    Word,
}

impl Expression {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Abstraction(k, e) => Kind::Abstraction(k.kind().into(), e.kind().into()),

            Self::Application(f, _) => match f.kind() {
                Kind::Abstraction(_, k) => k.as_ref().clone(),
                _ => unreachable!(),
            },

            Self::Argument(k) => k.clone(),

            Self::Boolean(_) => Kind::Boolean,

            Self::Class(c) => match c {
                Class::Comparable(_) => Kind::Boolean,
                Class::Equatable(_) => Kind::Boolean,
                Class::Negatable(Negatable::Negate(e)) => e.kind(),
                Class::Numerable(n) => n.kind(),
            },

            Self::Observable(o) => Kind::Observable(Box::new(match o {
                Observable::IsHolder => Kind::Boolean,
                Observable::IsCounterparty => Kind::Boolean,
                Observable::Konst(e) => e.kind(),
            })),

            Self::State(_) => Kind::State,

            Self::Word(_) => Kind::Word,
        }
    }
}

impl From<bool> for Expression {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<Class> for Expression {
    fn from(c: Class) -> Self {
        Self::Class(c)
    }
}

impl From<Kind> for Expression {
    fn from(k: Kind) -> Self {
        Self::Argument(k)
    }
}

impl From<Observable> for Expression {
    fn from(o: Observable) -> Self {
        Self::Observable(o)
    }
}

impl From<State> for Expression {
    fn from(s: State) -> Self {
        Self::State(s)
    }
}

impl From<u64> for Expression {
    fn from(w: u64) -> Self {
        Self::Word(w)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Observable {
    IsHolder,
    IsCounterparty,
    Konst(Rc<Expression>),
}

impl From<Expression> for Observable {
    fn from(e: Expression) -> Self {
        Self::Konst(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_of_application() {
        // one :: Word -> Word -> Boolean
        let one = Expression::Abstraction(
            Expression::Argument(Kind::Word).into(),
            Expression::Abstraction(
                Expression::Argument(Kind::Word).into(),
                Expression::Boolean(true).into(),
            )
            .into(),
        );

        // two = one 42
        let two = Expression::Application(one.into(), Expression::Word(42).into());

        // two :: Word -> Boolean
        assert_eq!(
            two.kind(),
            Kind::Abstraction(Kind::Word.into(), Kind::Boolean.into())
        );

        // three = two 29
        let three = Expression::Application(two.into(), Expression::Word(29).into());

        // three :: Boolean
        assert_eq!(three.kind(), Kind::Boolean);
    }
}
