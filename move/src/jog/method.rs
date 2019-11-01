use super::{action::Action, expression::Expression, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Default)]
pub struct Transition<'a> {
    conditions: Vec<Rc<Condition>>,
    actions: Vec<Box<dyn Action + 'a>>,
    origin_state: usize,
    to_state: usize,
    context: String,
}

impl<'a> Transition<'a> {
    pub fn new(origin_state: usize, to_state: usize, context: String) -> Self {
        Transition {
            conditions: Vec::new(),
            actions: Vec::new(),
            origin_state,
            to_state,
            context,
        }
    }

    pub fn dependencies(&self) -> Vec<&str> {
        self.actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
            .collect()
    }

    pub fn properties(&self) -> Vec<Rc<Variable>> {
        self.actions
            .iter()
            .flat_map(|action| action.properties())
            .collect()
    }

    pub fn definitions(&self) -> Vec<Rc<Variable>> {
        self.actions
            .iter()
            .flat_map(|action| action.definitions())
            .collect()
    }

    pub fn conditions(&self) -> &[Rc<Condition>] {
        self.conditions.as_slice()
    }

    pub fn add_condition(&mut self, condition: Rc<Condition>) {
        self.conditions.push(condition);
    }

    pub fn origin_state(&self) -> usize {
        self.origin_state
    }

    pub fn to_state(&self) -> usize {
        self.to_state
    }

    pub fn actions(&self) -> Vec<&dyn Action> {
        self.actions.iter().map(AsRef::as_ref).collect()
    }

    pub fn add_action(&mut self, action: impl Action + 'a) {
        self.actions.push(Box::new(action));
    }

    pub fn context(&self) -> &str {
        &self.context
    }
}

pub struct Condition {
    condition: Expression,
    error_code: u64,
}

impl Condition {
    pub fn new(condition: Expression, error_code: u64) -> Self {
        Condition {
            condition,
            error_code,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "assert({}, {});", self.condition, self.error_code)
    }
}
