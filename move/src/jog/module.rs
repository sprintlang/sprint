use super::{method::Transition, variable::Variable};
use askama::Template;
use std::{collections::HashSet, rc::Rc};

const DEPENDENCIES: [&str; 3] = ["0x0.Vector", "0x0.LibraCoin", "0x0.LibraAccount"];

#[derive(Template, Default)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    transition_methods: Vec<Transition<'a>>,
    properties: HashSet<Rc<Variable>>,
}

impl<'a> Contract<'a> {
    pub fn dependencies(&self) -> HashSet<&str> {
        let mut dependencies: HashSet<&str> = self
            .transition_methods
            .iter()
            .flat_map(|transition| transition.dependencies())
            .collect();

        for dependency in DEPENDENCIES.iter() {
            dependencies.insert(dependency);
        }

        dependencies
    }

    pub fn properties(&self) -> HashSet<Rc<Variable>> {
        let mut properties: HashSet<Rc<Variable>> = self
            .transition_methods
            .iter()
            .flat_map(|transition| transition.properties())
            .collect();

        for property in self.properties.iter() {
            properties.insert(property.clone());
        }

        properties
    }

    pub fn add_method(&mut self, method: Transition<'a>) {
        self.transition_methods.push(method);
    }
}
