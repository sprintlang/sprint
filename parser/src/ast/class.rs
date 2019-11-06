use super::{Expression, Kind};
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
pub enum Class {
    Comparable(Comparable),
    Equatable(Equatable),
    Negatable(Negatable),
    Numerable(Numerable),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Comparable {
    Greater(Rc<Expression>, Rc<Expression>),
    Less(Rc<Expression>, Rc<Expression>),
    GreaterEqual(Rc<Expression>, Rc<Expression>),
    LessEqual(Rc<Expression>, Rc<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Equatable {
    Equal(Rc<Expression>, Rc<Expression>),
    NotEqual(Rc<Expression>, Rc<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Negatable {
    Negate(Rc<Expression>),
}

#[derive(PartialEq, Eq, Debug)]
pub enum Numerable {
    Add(Rc<Expression>, Rc<Expression>),
    Subtract(Rc<Expression>, Rc<Expression>),
    Multiply(Rc<Expression>, Rc<Expression>),
    Divide(Rc<Expression>, Rc<Expression>),
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
