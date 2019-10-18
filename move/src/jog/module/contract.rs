use super::super::{method::Transition, variable::Variable};
use askama::Template;
use std::{collections::HashSet, rc::Rc};

#[derive(Template)]
#[template(path = "fsm.mvir", escape = "none")]
pub struct Contract<'a> {
    name: &'a str,
    transition_methods: Vec<Transition<'a>>,
    terminal_states: Vec<u64>,
}

impl<'a> Contract<'a> {
    pub fn new(name: &'a str) -> Self {
        Contract {
            name,
            transition_methods: Vec::new(),
            terminal_states: Vec::new(),
        }
    }

    pub fn dependencies(&self) -> HashSet<&str> {
        self.transition_methods
            .iter()
            .flat_map(|transition| transition.dependencies())
            .collect()
    }

    pub fn properties(&self) -> HashSet<Rc<Variable>> {
        self.transition_methods
            .iter()
            .flat_map(|transition| transition.properties())
            .collect()
    }

    pub fn add_method(&mut self, method: Transition<'a>) {
        self.transition_methods.push(method);
    }

    pub fn add_terminal_state(&mut self, state_id: u64) {
        self.terminal_states.push(state_id);
    }
}
