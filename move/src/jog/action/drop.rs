use super::{
    super::variable::{Variable, STACK},
    Action,
};
use lazy_static::lazy_static;
use std::fmt::{self, Display, Formatter};

lazy_static! {
    pub static ref DROP_STACK: Drop<'static> = Drop::new(STACK.clone());
}

#[derive(Clone, Debug)]
pub struct Drop<'a>(Variable<'a>);

impl<'a> Drop<'a> {
    pub fn new(drop: Variable<'a>) -> Self {
        Self(drop)
    }
}

impl<'a> Action for Drop<'a> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&self.0]
    }
}

impl Display for Drop<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "_ = move({});", self.0.identifier())
    }
}
