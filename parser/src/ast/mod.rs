pub mod state;

mod class;
mod kind;

pub use self::{
    class::{Class, Comparable, Equatable, Negatable, Numerable},
    kind::Kind,
};

use self::state::State;
use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

pub type Program<'a> = Vec<Definition<'a>>;

#[derive(Debug)]
pub struct Definition<'a> {
    pub variable: Variable<'a>,
    pub expression: Expression<'a>,
}

impl<'a> Definition<'a> {
    pub fn new(variable: Variable<'a>, expression: Expression<'a>) -> Self {
        Self {
            variable,
            expression,
        }
    }
}

#[derive(Debug)]
pub enum Expression<'a> {
    Abstraction(Variable<'a>, Box<Self>),
    Application(Box<Self>, Box<Self>),
    Boolean(bool),
    Class(Class<'a>),
    Observable(Observable<'a>),
    State(State<'a>),
    Variable(Variable<'a>),
    Word(u64),
}

impl Expression<'_> {
    pub fn kind(&self) -> Rc<Kind> {
        match self {
            Self::Abstraction(from, to) => Kind::Abstraction(from.kind.clone(), to.kind()).into(),

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
                Observable::IsParty => Kind::Boolean.into(),
                Observable::IsCounterparty => Kind::Boolean.into(),
                Observable::Konst(e) => e.kind(),
            })
            .into(),

            Self::State(_) => Kind::State.into(),

            Self::Variable(v) => v.kind.clone(),

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

impl<'a> From<Variable<'a>> for Expression<'a> {
    fn from(v: Variable<'a>) -> Self {
        Self::Variable(v)
    }
}

impl From<u64> for Expression<'_> {
    fn from(w: u64) -> Self {
        Self::Word(w)
    }
}

#[derive(Eq, Clone, Debug)]
pub struct Variable<'a> {
    pub name: &'a str,
    pub kind: Rc<Kind>,
}

impl<'a> Variable<'a> {
    pub fn new(name: &'a str, kind: Rc<Kind>) -> Self {
        Variable { name, kind }
    }
}

impl PartialEq for Variable<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Variable<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub enum Observable<'a> {
    IsParty,
    IsCounterparty,
    Konst(Rc<Expression<'a>>),
}

impl<'a> From<Expression<'a>> for Observable<'a> {
    fn from(e: Expression<'a>) -> Self {
        Self::Konst(e.into())
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
            Variable::new("x", Kind::Word.into()),
            Expression::Abstraction(
                Variable::new("y", Kind::Word.into()),
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
