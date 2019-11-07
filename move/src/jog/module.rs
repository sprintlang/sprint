use super::{method::Transition, variable::Variable};
use crate::jog::generate_context_name;
use askama::Template;
use std::{collections::HashSet, rc::Rc};

const DEPENDENCIES: [&str; 2] = ["0x0.LibraCoin", "0x0.LibraAccount"];

#[derive(Template)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    transition_methods: Vec<Transition<'a>>,
    properties: HashSet<Rc<Variable>>,
    initial_context: String,
}

impl<'a> Default for Contract<'a> {
    fn default() -> Contract<'a> {
        let mut properties = HashSet::new();

        // Bootstrap the initial context
        let initial_context = generate_context_name(0);
        properties.insert(Rc::new(Variable {
            name: initial_context.clone().into(),
            type_name: "Self.Context",
            default: Some("Context { state: 0, flipped: false, scale: 1 }".into()),
        }));

        Contract {
            transition_methods: vec![],
            properties,
            initial_context,
        }
    }
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
