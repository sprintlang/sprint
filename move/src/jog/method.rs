use super::{action::Action, variable::Variable};
use std::{collections::HashSet, rc::Rc};

#[derive(Default)]
pub struct Transition<'a> {
    actions: Vec<Box<dyn Action + 'a>>,
    origin_state: usize,
    to_state: usize,
}

impl<'a> Transition<'a> {
    pub fn new(origin_state: usize, to_state: usize) -> Self {
        Transition {
            actions: Default::default(),
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

    pub fn definitions(&self) -> HashSet<Rc<Variable>> {
        self.actions
            .iter()
            .flat_map(|action| action.definitions())
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
