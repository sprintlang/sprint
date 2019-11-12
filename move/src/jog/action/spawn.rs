use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Spawn {
    context: Rc<Variable>,
    root_state: usize,
}

impl Spawn {
    pub fn new(root_state: usize) -> Self {
        Spawn {
            context: Rc::new(Variable {
                // TODO: Make this random name gen to allow multiple spawns
                // in the same transition method
                name: "spawned_context".into(),
                type_name: "Self.Context",
                default: None,
            }),
            root_state,
        }
    }

    pub fn spawned_context(&self) -> Rc<Variable> {
        self.context.clone()
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
                flipped: *(&copy(context_ref).flipped),
                scale: *(&copy(context_ref).scale),
            }};",
            self.context.name, self.root_state,
        )
    }
}
