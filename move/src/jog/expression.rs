use super::{call::Call, identifier::Identifier, kind::Kind};
use std::{
    borrow::Cow,
    cell::RefCell,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Add(Box<Self>, Box<Self>),
    Call(Call<'a>),
    Copied(Box<Self>),
    Expression(Cow<'static, str>),
    Get(Kind, Box<Self>, Box<Self>),
    Identifier(Identifier<'a>),
    Length(Kind, Box<Self>),
    Moved(Box<Self>),
    MutableReference(Box<Self>),
    Reference(Box<Self>),
    State(Rc<RefCell<Option<usize>>>),
    Subtract(Box<Self>, Box<Self>),
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
            Self::Add(l, r) => write!(f, "{} + {}", l, r),
            Self::Call(c) => c.fmt(f),
            Self::Copied(e) => write!(f, "copy({})", e),
            Self::Expression(e) => e.fmt(f),
            Self::Get(k, v, i) => write!(f, "Vector.get<{}>({}, {})", k, v, i),
            Self::Identifier(i) => i.fmt(f),
            Self::Length(k, v) => write!(f, "Vector.length<{}>({})", k, v),
            Self::Moved(e) => write!(f, "move({})", e),
            Self::MutableReference(e) => write!(f, "&mut {}", e),
            Self::Reference(e) => write!(f, "&{}", e),
            Self::State(u) => u.borrow().unwrap().fmt(f),
            Self::Subtract(l, r) => write!(f, "{} - {}", l, r),
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

#[derive(Debug)]
pub enum Address {
    Party,
    Counterparty,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Address::Party => write!(f, "copy(context_ref).party"),
            Address::Counterparty => write!(f, "copy(context_ref).counterparty"),
        }
    }
}
