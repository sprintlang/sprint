use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Spawn {
    context: Rc<Variable>,
    context_ref: Rc<Variable>,
}

impl Spawn {
    pub fn new(context_name: &str, root_state: usize) -> Self {
        let type_name = "Self.Context";

        Spawn {
            context: Rc::new(Variable {
                name: String::from(context_name),
                type_name,
                default: Some(format!(
                    "Context {{ state: {}, flipped: false, scale: 1 }}",
                    root_state
                )),
            }),
            context_ref: Rc::new(Variable {
                name: String::from(context_name),
                type_name,
                default: Some(format!("&mut copy(contract_ref).{}", context_name)),
            }),
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
        vec![self.context.clone()]
    }
}

impl Display for Spawn {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
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
