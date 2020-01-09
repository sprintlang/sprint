pub mod assert;
pub mod assign;
pub mod call;
pub mod drop;
pub mod flip;
pub mod libra;
pub mod push;
pub mod scale;
pub mod spawn;
pub mod update_state;

use super::variable::Variable;
use std::fmt::{Debug, Display};

pub trait Action: Display + Debug {
    fn dependencies(&self) -> &'static [&'static str];

    fn definitions(&self) -> Vec<&Variable>;
}
