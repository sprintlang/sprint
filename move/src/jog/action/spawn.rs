use crate::jog::{
    action::{libra::Address, Action},
    variable::Variable,
};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Spawn {
    context: Rc<Variable>,
    root_state: usize,
}

impl Spawn {
    pub fn new(context: Rc<Variable>, root_state: usize) -> Self {
        Spawn {
            context,
            root_state,
        }
    }
}

impl Action for Spawn {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![self.context.clone()]
    }
}

impl Display for Spawn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} = Context {{
                state: {},
                holder: {},
                counterparty: {},
                scale: *(&copy(context_ref).scale),
            }};",
            self.context.name,
            self.root_state,
            Address::Holder,
            Address::Counterparty,
        )
    }
}

pub struct PushContext {
    context: Rc<Variable>,
}

impl PushContext {
    pub fn new(context: Rc<Variable>) -> Self {
        PushContext { context }
    }
}

impl Action for PushContext {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![self.context.clone()]
    }
}

impl Display for PushContext {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "Vector.push_back<Self.Context>(copy(contexts), move({}));",
            self.context.name
        )
    }
}
