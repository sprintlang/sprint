use super::{action::Action, variable::Variable};
use std::{fmt, rc::Rc};

#[derive(Default)]
pub struct Transition<'a> {
    actions: Vec<Box<dyn Action + 'a>>,
    origin_state: usize,
    to_state: usize,
}

impl<'a> Transition<'a> {
    pub fn new(origin_state: usize, to_state: usize) -> Self {
        Transition {
            actions: Vec::new(),
            origin_state,
            to_state,
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

    pub fn conditions(&self) -> Vec<Rc<Condition>> {
        self.actions
            .iter()
            .flat_map(|action| action.conditions())
            .collect()
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
}

pub struct Condition {
    check: String,
    error_code: u64,
}

impl Condition {
    pub fn new(check: String, error_code: u64) -> Self {
        Condition { check, error_code }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "assert({}, {});", self.check, self.error_code)
    }
}
