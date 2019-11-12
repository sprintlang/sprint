pub mod flip;
pub mod libra;
pub mod scale;
pub mod spawn;
pub mod update_state;

use super::variable::Variable;
use std::{fmt::Display, rc::Rc};

pub trait Action: Display {
    fn dependencies(&self) -> &'static [&'static str];

    fn properties(&self) -> Vec<Rc<Variable>>;

    fn definitions(&self) -> Vec<Rc<Variable>>;
}
