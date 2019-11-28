use super::{application::Application, identifier::Identifier};
use std::{
    borrow::Cow,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Application(Application<'a>),
    Expression(Cow<'static, str>),
    Identifier(Identifier<'a>),
    Unsigned(usize),
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Self::Expression(Cow::default())
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Application(a) => a.fmt(f),
            Self::Expression(e) => e.fmt(f),
            Self::Identifier(i) => i.fmt(f),
            Self::Unsigned(u) => u.fmt(f),
        }
    }
}

impl<'a> From<Application<'a>> for Expression<'a> {
    fn from(a: Application<'a>) -> Self {
        Self::Application(a)
    }
}

impl From<usize> for Expression<'_> {
    fn from(n: usize) -> Self {
        Self::Unsigned(n)
    }
}

impl<'a> From<Identifier<'a>> for Expression<'a> {
    fn from(i: Identifier<'a>) -> Self {
        Self::Identifier(i)
    }
}

impl<'a> TryFrom<Expression<'a>> for usize {
    type Error = Expression<'a>;

    fn try_from(expression: Expression<'a>) -> Result<Self, Self::Error> {
        if let Expression::Unsigned(u) = expression {
            return Ok(u);
        }

        Err(expression)
    }
}

pub enum Address {
    Holder,
    Counterparty,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Address::Holder => write!(f, "*(&copy(context_ref).holder)"),
            Address::Counterparty => write!(f, "*(&copy(context_ref).counterparty)"),
        }
    }
}
