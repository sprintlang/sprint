use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

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

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for UpdateState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "*(&mut move(context_ref).state) = {};", self.new_state,)
    }
}
