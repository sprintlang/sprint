use crate::jog::{action::Action, variable::Variable};
use std::fmt::{self, Display, Formatter};

pub struct UpdateState {
    new_state: usize,
}

impl UpdateState {
    pub fn new(new_state: usize) -> Self {
        UpdateState { new_state }
    }
}

impl Action for UpdateState {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for UpdateState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "*(&mut move(context_ref).state) = {};", self.new_state)
    }
}
