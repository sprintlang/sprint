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

pub type Reference<'a> = Rc<RefCell<Variable<'a>>>;

#[derive(Debug)]
pub struct Definition<'a> {
    pub name: &'a str,
    pub kind: Rc<Kind>,
    pub expression: Expression<'a>,
}

impl<'a> Definition<'a> {
    pub fn new(name: &'a str, expression: Expression<'a>) -> Self {
        Self {
            name,
            kind: expression.kind(),
            expression,
        }
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Abstraction(Rc<Argument<'a>>, Box<Self>),
    Application(Box<Self>, Box<Self>),
    Boolean(bool),
    Class(Class<'a>),
    Observable(Observable<'a>),
    State(State<'a>),
    Variable(Reference<'a>),
    Word(u64),
}

impl Expression<'_> {
    pub fn kind(&self) -> Rc<Kind> {
        match self {
            Self::Abstraction(from, to) => {
                Kind::Abstraction(from.as_ref().kind.clone(), to.kind()).into()
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

impl From<bool> for Expression<'_> {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl<'a> From<Class<'a>> for Expression<'a> {
    fn from(c: Class<'a>) -> Self {
        Self::Class(c)
    }
}

impl<'a> From<Observable<'a>> for Expression<'a> {
    fn from(o: Observable<'a>) -> Self {
        Self::Observable(o)
    }
}

impl<'a> From<State<'a>> for Expression<'a> {
    fn from(s: State<'a>) -> Self {
        Self::State(s)
    }
}

impl<'a> From<Rc<RefCell<Variable<'a>>>> for Expression<'a> {
    fn from(v: Rc<RefCell<Variable<'a>>>) -> Self {
        Self::Variable(v)
    }
}

impl From<u64> for Expression<'_> {
    fn from(w: u64) -> Self {
        Self::Word(w)
    }
}

#[derive(Debug)]
pub struct Argument<'a> {
    pub name: &'a str,
    pub kind: Rc<Kind>,
}

impl<'a> Argument<'a> {
    pub fn new(name: &'a str, kind: Rc<Kind>) -> Self {
        Self { name, kind }
    }
}

#[derive(Debug)]
pub enum Observable<'a> {
    IsHolder,
    IsCounterparty,
    Konst(Rc<Expression<'a>>),
}

impl<'a> From<Expression<'a>> for Observable<'a> {
    fn from(e: Expression<'a>) -> Self {
        Self::Konst(e.into())
    }
}

#[derive(Clone, Debug)]
pub enum Variable<'a> {
    Argument(Rc<Argument<'a>>),
    Definition(Weak<Definition<'a>>),
    Undefined(Rc<Kind>),
}

impl Variable<'_> {
    pub fn kind(&self) -> Rc<Kind> {
        match self {
            Self::Argument(a) => a.kind.clone(),
            Self::Definition(d) => d.upgrade().unwrap().kind.clone(),
            Self::Undefined(k) => k.clone(),
        }
    }
}

impl<'a> From<Rc<Argument<'a>>> for Variable<'a> {
    fn from(a: Rc<Argument<'a>>) -> Self {
        Self::Argument(a)
    }
}

impl From<Rc<Kind>> for Variable<'_> {
    fn from(k: Rc<Kind>) -> Self {
        Self::Undefined(k)
    }
}

impl<'a> From<&Rc<Definition<'a>>> for Variable<'a> {
    fn from(d: &Rc<Definition<'a>>) -> Self {
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
            Argument::new("x", Kind::Word.into()).into(),
            Expression::Abstraction(
                Argument::new("y", Kind::Word.into()).into(),
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
