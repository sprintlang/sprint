use super::{method::Transition, variable::Variable};
use askama::Template;
use std::{collections::HashSet, rc::Rc};

const DEPENDENCIES: [&str; 2] = ["0x0.LibraCoin", "0x0.LibraAccount"];

#[derive(Template)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    transition_methods: Vec<Transition<'a>>,
    properties: HashSet<Rc<Variable>>,
    last_used_context_id: usize,
    initial_context: &'static str,
}

impl<'a> Default for Contract<'a> {
    fn default() -> Contract<'a> {
        let mut properties = HashSet::new();

        // Initial context
        properties.insert(Rc::new(Variable {
            name: "context_0".into(),
            type_name: "Self.Context",
            default: Some(
                "Context {
                    state: 0,
                    flipped: false,
                    scale: 1,
                }"
                .into(),
            ),
        }));

        Contract {
            transition_methods: vec![],
            properties,
            last_used_context_id: 0,
            initial_context: "context_0",
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

    pub fn next_context(&mut self) -> String {
        self.last_used_context_id += 1;
        format!("context_{}", self.last_used_context_id)
    }

    pub fn initial_context(&self) -> String {
        self.initial_context.into()
    }
}
