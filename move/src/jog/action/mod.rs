pub mod flip;
pub mod libra;

use super::{method::Condition, variable::Variable};
use std::{fmt::Display, rc::Rc};

pub trait Action: Display {
    fn dependencies(&self) -> &'static [&'static str];

    fn properties(&self) -> Vec<Rc<Variable>>;

    fn definitions(&self) -> Vec<Rc<Variable>>;

    fn conditions(&self) -> Vec<Rc<Condition>>;
}
