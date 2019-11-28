use crate::jog::{action::Action, expression::Expression, variable::Variable};
use std::fmt::{self, Display, Formatter};

pub struct UpdateState<'a> {
    to: Expression<'a>,
}

impl<'a> UpdateState<'a> {
    pub fn new(to: Expression<'a>) -> Self {
        UpdateState { to }
    }
}

impl Action for UpdateState<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for UpdateState<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "*(&mut move(context_ref).state) = {};", self.to)
    }
}
