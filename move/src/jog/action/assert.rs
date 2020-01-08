use super::{
    super::{expression::Expression, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Default, Debug)]
pub struct Assert<'a> {
    predicate: Expression<'a>,
    code: u64,
}

impl<'a> Assert<'a> {
    pub fn new(predicate: Expression<'a>, code: u64) -> Self {
        Self { predicate, code }
    }
}

impl Action for Assert<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for Assert<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "assert({}, {});", self.predicate, self.code)
    }
}
