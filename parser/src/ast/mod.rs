pub mod state;

mod class;
mod date;
mod kind;

pub use self::{
    class::{Class, Comparable, Equatable, Negatable, Numerable},
    date::Date,
    kind::Kind,
    state::State,
};

use super::parser::Span;
use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

pub type Definitions<'a> = Vec<Definition<'a>>;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Expression<'a> {
    pub expression: ExpressionType<'a>,
    pub span: Option<Span<'a>>,
}

impl<'a> Expression<'a> {
    pub fn new(expression: ExpressionType<'a>, span: Option<Span<'a>>) -> Self {
        Self { expression, span }
    }

    pub fn kind(&self) -> Rc<Kind> {
        self.expression.kind()
    }
}

#[derive(Clone, Debug)]
pub enum ExpressionType<'a> {
    Abstraction(Variable<'a>, Box<Expression<'a>>),
    Application(Box<Expression<'a>>, Box<Expression<'a>>),
    Boolean(bool),
    Class(Class<'a>),
    Date(Date),
    Observable(Observable<'a>),
    State(State<'a>),
    Variable(Variable<'a>),
    Word(u64),
}

impl ExpressionType<'_> {
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

            Self::Date(_) => Kind::Date.into(),

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

impl From<bool> for ExpressionType<'_> {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl<'a> From<Class<'a>> for ExpressionType<'a> {
    fn from(c: Class<'a>) -> Self {
        Self::Class(c)
    }
}

impl<'a> From<Observable<'a>> for ExpressionType<'a> {
    fn from(o: Observable<'a>) -> Self {
        Self::Observable(o)
    }
}

impl<'a> From<State<'a>> for ExpressionType<'a> {
    fn from(s: State<'a>) -> Self {
        Self::State(s)
    }
}

impl<'a> From<Variable<'a>> for ExpressionType<'a> {
    fn from(v: Variable<'a>) -> Self {
        Self::Variable(v)
    }
}

impl From<u64> for ExpressionType<'_> {
    fn from(w: u64) -> Self {
        Self::Word(w)
    }
}

#[derive(Clone, Debug)]
pub struct Variable<'a> {
    pub name: &'a str,
    pub kind: Rc<Kind>,
    pub span: Option<Span<'a>>,
}

impl<'a> Variable<'a> {
    pub fn new(name: &'a str, kind: Rc<Kind>, span: Option<Span<'a>>) -> Self {
        Variable { name, kind, span }
    }
}

impl PartialEq for Variable<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Variable<'_> {}

impl Hash for Variable<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Clone, Debug)]
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
        let one = Expression::new(
            ExpressionType::Abstraction(
                Variable::new("x", Kind::Word.into(), None),
                Expression::new(
                    ExpressionType::Abstraction(
                        Variable::new("y", Kind::Word.into(), None),
                        Expression::new(ExpressionType::Boolean(true), None).into(),
                    ),
                    None,
                )
                .into(),
            ),
            None,
        );

        // two = one 42
        let two = Expression::new(
            ExpressionType::Application(
                one.into(),
                Expression::new(ExpressionType::Word(42), None).into(),
            ),
            None,
        );

        // two :: Word -> Boolean
        assert_eq!(
            *two.kind(),
            Kind::Abstraction(Kind::Word.into(), Kind::Boolean.into())
        );

        // three = two 29
        let three = Expression::new(
            ExpressionType::Application(
                two.into(),
                Expression::new(ExpressionType::Word(29), None).into(),
            ),
            None,
        );

        // three :: Boolean
        assert_eq!(*three.kind(), Kind::Boolean);
    }
}
