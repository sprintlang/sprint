use super::{action::Action, variable::Variable};
use std::rc::Rc;

#[derive(Default)]
pub struct Method {
    actions: Vec<Box<dyn Action>>,
}

impl Method {
    pub fn dependencies(&self) -> Vec<&&str> {
        self.actions
            .iter()
            .flat_map(|action| action.dependencies())
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

    pub fn actions(&self) -> &[Box<dyn Action>] {
        &self.actions
    }

    pub fn add_action(&mut self, action: Box<dyn Action>) {
        self.actions.push(action);
    }
}
