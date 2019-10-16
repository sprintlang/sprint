use super::{action::Action, variable::Variable};
use std::rc::Rc;

#[derive(Default)]
pub struct Method {
    actions: Vec<Box<dyn Action>>,
}

    pub fn dependencies(&self) -> impl Iterator<Item = &str> {
        self.actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
    }

    pub fn properties(&self) -> impl Iterator<Item = Rc<Variable>> + '_ {
        self.actions.iter().flat_map(|action| action.properties())
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
