use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

const CURRENT_FLIPPED_STATE_VAR: &str = "current_flipped_state";
const CURRENT_SCALE_STATE_VAR: &str = "current_scale_state";

#[derive(Default)]
pub struct SpawnSetup;

impl Action for SpawnSetup {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![
            Rc::new(Variable {
                name: CURRENT_FLIPPED_STATE_VAR.into(),
                type_name: "bool",
                default: None,
            }),
            Rc::new(Variable {
                name: CURRENT_SCALE_STATE_VAR.into(),
                type_name: "u64",
                default: None,
            }),
        ]
    }
}

impl Display for SpawnSetup {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} = *(&copy(context_ref).flipped);",
            CURRENT_FLIPPED_STATE_VAR
        )?;
        writeln!(
            f,
            "{} = *(&copy(context_ref).scale);",
            CURRENT_SCALE_STATE_VAR
        )
    }
}

pub struct Spawn {
    context: Rc<Variable>,
    root_state: usize,
}

impl Spawn {
    pub fn new(root_state: usize) -> Self {
        Spawn {
            context: Rc::new(Variable {
                name: "spawned_context".into(),
                type_name: "Self.Context",
                default: None,
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
                flipped: move({}),
                scale: move({}),
            }};",
            self.context.name, self.root_state, CURRENT_FLIPPED_STATE_VAR, CURRENT_SCALE_STATE_VAR,
        )?;
        write!(
            f,
            "Vector.push_back<Self.Context>(copy(contexts), move({}));",
            self.context.name
        )
    }
}
