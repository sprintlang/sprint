use super::{
    super::{expression::Expression, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Assign<'a> {
    variable: Variable<'a>,
    value: Expression<'a>,
}

impl<'a> Assign<'a> {
    pub fn new(variable: Variable<'a>, value: Expression<'a>) -> Self {
        Self { variable, value }
    }
}

impl Action for Assign<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&self.variable]
    }
}

impl Display for Assign<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} = {};", self.variable.identifier(), self.value)
    }
}
