pub mod class;
pub mod kind;
pub mod visitor;

use self::{kind::Kind, visitor::Accept};
use std::fmt::Debug;

pub trait Expression: Accept + Debug {
    fn kind(&self) -> Kind;
}
