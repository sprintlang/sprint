use super::{method::Transition, variable::Variable};
use askama::Template;
use std::{collections::HashSet, rc::Rc};

#[derive(Default, Template)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    transition_methods: Vec<Transition<'a>>,
}

impl<'a> Contract<'a> {
    #[allow(dead_code)]
    pub fn dependencies(&self) -> HashSet<&str> {
        self.transition_methods
            .iter()
            .flat_map(|transition| transition.dependencies())
            .collect()
    }

    #[allow(dead_code)]
    pub fn properties(&self) -> HashSet<Rc<Variable>> {
        self.transition_methods
            .iter()
            .flat_map(|transition| transition.properties())
            .collect()
    }

    pub fn add_method(&mut self, method: Transition<'a>) {
        self.transition_methods.push(method);
    }
}
