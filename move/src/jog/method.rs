use super::{action::Action, variable::Variable};
use std::rc::Rc;

#[derive(Default)]
pub struct Method<'a> {
    actions: Vec<Box<dyn Action + 'a>>,
}

impl<'a> Method<'a> {
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

    pub fn actions(&self) -> Vec<&dyn Action> {
        self.actions.iter().map(AsRef::as_ref).collect()
    }

    pub fn add_action(&mut self, action: impl Action + 'a) {
        self.actions.push(Box::new(action));
    }
}
