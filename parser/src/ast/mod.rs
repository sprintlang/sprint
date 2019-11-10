pub mod state;

mod class;
mod kind;

pub use self::{
    class::{Class, Comparable, Equatable, Negatable, Numerable},
    kind::Kind,
};

use self::state::State;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

pub type Reference = Rc<RefCell<Variable>>;

#[derive(Debug)]
pub struct Definition {
    pub kind: Rc<Kind>,
    pub expression: Expression,
}

impl From<Expression> for Definition {
    fn from(e: Expression) -> Self {
        Self {
            kind: e.kind(),
            expression: e,
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Abstraction(Rc<Argument>, Box<Self>),
    Application(Box<Self>, Box<Self>),
    Boolean(bool),
    Class(Class),
    Observable(Observable),
    State(State),
    Variable(Reference),
    Word(u64),
}

impl Expression {
    pub fn kind(&self) -> Rc<Kind> {
        match self {
            Self::Abstraction(from, to) => {
                let Argument(from) = from.as_ref();
                Kind::Abstraction(from.clone(), to.kind()).into()
            }

            Self::Application(f, _) => match f.kind().as_ref() {
                Kind::Abstraction(_, k) => k.clone(),
                _ => unreachable!(),
            },

            Self::Boolean(_) => Kind::Boolean.into(),

            Self::Class(c) => match c {
                Class::Comparable(_) => Kind::Boolean.into(),
                Class::Equatable(_) => Kind::Boolean.into(),
                Class::Negatable(Negatable::Negate(e)) => e.kind(),
                Class::Numerable(n) => n.kind(),
            },

            Self::Observable(o) => Kind::Observable(match o {
                Observable::IsHolder => Kind::Boolean.into(),
                Observable::IsCounterparty => Kind::Boolean.into(),
                Observable::Konst(e) => e.kind(),
            })
            .into(),

            Self::State(_) => Kind::State.into(),

            Self::Variable(v) => v.borrow().kind(),

            Self::Word(_) => Kind::Word.into(),
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

impl From<Rc<RefCell<Variable>>> for Expression {
    fn from(v: Rc<RefCell<Variable>>) -> Self {
        Self::Variable(v)
    }
}

impl From<u64> for Expression {
    fn from(w: u64) -> Self {
        Self::Word(w)
    }
}

#[derive(Debug)]
pub struct Argument(pub Rc<Kind>);

impl From<Rc<Kind>> for Argument {
    fn from(k: Rc<Kind>) -> Self {
        Argument(k)
    }
}

#[derive(Debug)]
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

#[derive(Clone, Debug)]
pub enum Variable {
    Argument(Rc<Argument>),
    Definition(Weak<Definition>),
    Undefined(Rc<Kind>),
}

impl Variable {
    pub fn kind(&self) -> Rc<Kind> {
        match self {
            Self::Argument(a) => {
                let Argument(a) = a.as_ref();
                a.clone()
            }
            Self::Definition(d) => d.upgrade().unwrap().kind.clone(),
            Self::Undefined(k) => k.clone(),
        }
    }
}

impl From<Rc<Argument>> for Variable {
    fn from(a: Rc<Argument>) -> Self {
        Self::Argument(a)
    }
}

impl From<Rc<Kind>> for Variable {
    fn from(k: Rc<Kind>) -> Self {
        Self::Undefined(k)
    }
}

impl From<&Rc<Definition>> for Variable {
    fn from(d: &Rc<Definition>) -> Self {
        Self::Definition(Rc::downgrade(d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_of_application() {
        // one :: Word -> Word -> Boolean
        // one x y = true
        let one = Expression::Abstraction(
            Argument(Kind::Word.into()).into(),
            Expression::Abstraction(
                Argument(Kind::Word.into()).into(),
                Expression::Boolean(true).into(),
            )
            .into(),
        );

        // two = one 42
        let two = Expression::Application(one.into(), Expression::Word(42).into());

        // two :: Word -> Boolean
        assert_eq!(
            *two.kind(),
            Kind::Abstraction(Kind::Word.into(), Kind::Boolean.into())
        );

        // three = two 29
        let three = Expression::Application(two.into(), Expression::Word(29).into());

        // three :: Boolean
        assert_eq!(*three.kind(), Kind::Boolean);
    }
}
