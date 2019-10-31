use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Default)]
pub struct Scale {
    expression: String,
}

impl Scale {
    pub fn new(expression: String) -> Self {
        Scale { expression }
    }
}

impl Action for Scale {
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

impl Display for Scale {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "*(&mut copy(contract_ref).scale) = *(&copy(contract_ref).scale) * {}",
            self.expression
        )
    }
}
