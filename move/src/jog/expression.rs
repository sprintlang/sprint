use super::{call::Call, identifier::Identifier, kind::Kind};
use crate::numbers::Numbers;
use std::{
    borrow::Cow,
    cell::RefCell,
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Clone, Debug)]
pub enum Expression<'a> {
    Binary(Binary, Box<Self>, Box<Self>),
    Call(Call<'a>),
    Copy(Box<Self>),
    Expression(Cow<'static, str>),
    Freeze(Box<Self>),
    Get(Kind, Box<Self>, Box<Self>),
    Identifier(Identifier<'a>),
    Length(Kind, Box<Self>),
    Move(Box<Self>),
    MutableReference(Box<Self>),
    Numbers(Rc<RefCell<Numbers>>),
    Observable(&'a str),
    Reference(Box<Self>),
    State(Rc<RefCell<Option<u64>>>),
    Unsigned(u64),
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Self::Expression(Cow::default())
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Binary(b, l, r) => write!(f, "({} {} {})", l, b, r),
            Self::Call(c) => c.fmt(f),
            Self::Copy(e) => write!(f, "copy({})", e),
            Self::Expression(e) => e.fmt(f),
            Self::Freeze(e) => write!(f, "freeze({})", e),
            Self::Get(k, v, i) => write!(f, "Vector.get<{}>({}, {})", k, v, i),
            Self::Identifier(i) => i.fmt(f),
            Self::Length(k, v) => write!(f, "Vector.length<{}>({})", k, v),
            Self::Move(e) => write!(f, "move({})", e),
            Self::MutableReference(e) => write!(f, "&mut {}", e),
            Self::Numbers(n) => n.borrow().peek().fmt(f),
            Self::Observable(o) => write!(f, "{}.get_value({{{{alice}}}})", o),
            Self::Reference(e) => write!(f, "&{}", e),
            Self::State(u) => u.borrow().unwrap().fmt(f),
            Self::Unsigned(u) => u.fmt(f),
        }
    }
}

impl<'a> From<Call<'a>> for Expression<'a> {
    fn from(c: Call<'a>) -> Self {
        Self::Call(c)
    }
}

impl From<u64> for Expression<'_> {
    fn from(n: u64) -> Self {
        Self::Unsigned(n)
    }
}

impl<'a> From<Identifier<'a>> for Expression<'a> {
    fn from(i: Identifier<'a>) -> Self {
        Self::Identifier(i)
    }
}

impl<'a> Expression<'a> {
    pub fn copy(self) -> Self {
        Expression::Copy(self.into())
    }

    pub fn freeze(self) -> Self {
        Expression::Freeze(self.into())
    }

    pub fn r#move(self) -> Self {
        Expression::Move(self.into())
    }

    pub fn mutable_reference(self) -> Self {
        Expression::MutableReference(self.into())
    }

    pub fn reference(self) -> Self {
        Expression::Reference(self.into())
    }

    /// Rewrites vector get expressions into calls to expand_stack. Leaves all other expressions
    /// unmodified.
    pub fn stack_expansion(self) -> Self {
        if let Expression::Get(Kind::Unsigned, f, i) = &self {
            if let Expression::Freeze(v) = f.as_ref() {
                return Expression::Expression(format!("Self.expand_stack({}, {})", v, i).into());
            }
        }

        self
    }
}

impl<'a> TryFrom<Expression<'a>> for u64 {
    type Error = Expression<'a>;

    fn try_from(expression: Expression<'a>) -> Result<Self, Self::Error> {
        if let Expression::Unsigned(u) = expression {
            return Ok(u);
        }

        Err(expression)
    }
}

#[derive(Clone, Debug)]
pub enum Binary {
    Add,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Subtract,
}

impl Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Binary::Add => write!(f, "+"),
            Binary::Greater => write!(f, ">"),
            Binary::GreaterEqual => write!(f, ">="),
            Binary::Less => write!(f, "<"),
            Binary::LessEqual => write!(f, "<="),
            Binary::Subtract => write!(f, "-"),
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Party,
    Counterparty,
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Address::Party => write!(f, "copy(context_ref).party"),
            Address::Counterparty => write!(f, "copy(context_ref).counterparty"),
        }
    }
}
