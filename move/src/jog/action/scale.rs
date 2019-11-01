use super::{
    super::{expression::Expression, variable::Variable},
    Action,
};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Scale {
    scalar: Expression,
}

impl Scale {
    pub fn new(scalar: Expression) -> Self {
        Scale { scalar }
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
            self.scalar
        )
    }
}
