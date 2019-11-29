use super::{call::Call, identifier::Identifier};
use std::{
    borrow::Cow,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
};

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Call(Call<'a>),
    Expression(Cow<'static, str>),
    Identifier(Identifier<'a>),
    MovedMutableReference(Identifier<'a>),
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
            Self::Call(c) => c.fmt(f),
            Self::Expression(e) => e.fmt(f),
            Self::Identifier(i) => i.fmt(f),
            Self::MovedMutableReference(i) => write!(f, "&mut move({})", i),
            Self::Unsigned(u) => u.fmt(f),
        }
    }
}

impl<'a> From<Call<'a>> for Expression<'a> {
    fn from(c: Call<'a>) -> Self {
        Self::Call(c)
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
