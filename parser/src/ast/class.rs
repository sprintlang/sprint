use super::{Expression, Kind};

#[derive(PartialEq, Eq, Debug)]
pub enum Class<'a> {
    Comparable(Comparable<'a>),
    Equatable(Equatable<'a>),
    Negatable(Negatable<'a>),
    Numerable(Numerable<'a>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Comparable<'a> {
    Greater(Box<Expression<'a>>, Box<Expression<'a>>),
    Less(Box<Expression<'a>>, Box<Expression<'a>>),
    GreaterEqual(Box<Expression<'a>>, Box<Expression<'a>>),
    LessEqual(Box<Expression<'a>>, Box<Expression<'a>>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Equatable<'a> {
    Equal(Box<Expression<'a>>, Box<Expression<'a>>),
    NotEqual(Box<Expression<'a>>, Box<Expression<'a>>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Negatable<'a> {
    Negate(Box<Expression<'a>>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Numerable<'a> {
    Add(Box<Expression<'a>>, Box<Expression<'a>>),
    Subtract(Box<Expression<'a>>, Box<Expression<'a>>),
    Multiply(Box<Expression<'a>>, Box<Expression<'a>>),
    Divide(Box<Expression<'a>>, Box<Expression<'a>>),
}

impl Numerable<'_> {
    pub(super) fn kind(&self) -> Kind {
        match self {
            Self::Add(e, _) | Self::Subtract(e, _) | Self::Multiply(e, _) | Self::Divide(e, _) => {
                e.kind()
            }
        }
    }
}
