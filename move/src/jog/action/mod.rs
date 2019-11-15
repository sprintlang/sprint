pub mod assert;
pub mod assign;
pub mod call;
pub mod flip;
pub mod libra;
pub mod scale;
pub mod spawn;
pub mod update_state;

use super::variable::Variable;
use std::fmt::Display;

pub trait Action: Display {
  fn dependencies(&self) -> &'static [&'static str];

  fn definitions(&self) -> Vec<&Variable>;
}
