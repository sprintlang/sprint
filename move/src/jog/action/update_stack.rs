use crate::jog::{
    action::Action,
    expression::Expression,
    variable::{Variable, STACK},
};
use std::fmt::{self, Display, Formatter};

#[derive(Default, Debug)]
pub struct UpdateStack();

impl Action for UpdateStack {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for UpdateStack {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "*(&mut move(context_ref).stack) = {};",
            Expression::Moved(Expression::Identifier(STACK.identifier().clone()).into())
        )
    }
}
