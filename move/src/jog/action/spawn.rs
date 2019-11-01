use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Spawn {
    context: Rc<Variable>,
    context_ref: Rc<Variable>,
    root_state: usize,
}

impl Spawn {
    pub fn new(context_name: &str, root_state: usize) -> Self {
        let type_name = "Self.Context";

        Spawn {
            context: Rc::new(Variable {
                name: String::from(context_name),
                type_name,
                default: Some(format!("Context {{ state: 0, flipped: false, scale: 1 }}",)),
            }),
            context_ref: Rc::new(Variable {
                name: String::from(context_name),
                type_name,
                default: Some(format!("&mut copy(contract_ref).{}", context_name)),
            }),
            root_state,
        }
    }
}

impl Action for Spawn {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![self.context.clone()]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![self.context_ref.clone()]
    }
}

impl Display for Spawn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "*(&mut copy({new_context_ref}).state) = {root_state};",
            new_context_ref = self.context_ref.name,
            root_state = self.root_state,
        )?;
        writeln!(
            f,
            "*(&mut copy({new_context_ref}).flipped) = *(&copy(context_ref).flipped);",
            new_context_ref = self.context_ref.name
        )?;
        write!(
            f,
            "*(&mut copy({new_context_ref}).scale) = *(&copy(context_ref).scale);",
            new_context_ref = self.context_ref.name
        )
    }
}
