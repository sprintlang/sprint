use super::{Expression, Kind};

#[derive(PartialEq, Eq, Debug)]
pub enum Class {
    Comparable(Comparable),
    Equatable(Equatable),
    Negatable(Negatable),
    Numerable(Numerable),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Comparable {
    Greater(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Equatable {
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Negatable {
    Negate(Box<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Numerable {
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
}

impl Numerable {
    pub(super) fn kind(&self) -> Kind {
        match self {
            Self::Add(e, _) | Self::Subtract(e, _) | Self::Multiply(e, _) | Self::Divide(e, _) => {
                e.kind()
            }
        }
    }
}
