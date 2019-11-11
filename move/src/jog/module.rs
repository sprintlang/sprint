use super::{method::Transition, variable::Variable};
use askama::Template;
use std::{collections::HashMap, collections::HashSet, rc::Rc};

const DEPENDENCIES: [&str; 2] = ["0x0.LibraCoin", "0x0.LibraAccount"];

#[derive(Template)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    transition_methods: Vec<Transition<'a>>,
    properties: HashSet<Rc<Variable>>,
    contexts: HashMap<usize, Rc<Variable>>,
    initial_context: &'static str,
}

impl<'a> Default for Contract<'a> {
    fn default() -> Contract<'a> {
        let mut properties = HashSet::new();
        let initial_context = "context_0";

        let initial_context_var = Rc::new(Variable {
            name: initial_context.into(),
            type_name: "Self.Context",
            default: Some("Context { state: 0, flipped: false, scale: 1 }".into()),
        });

        // Bootstrap the initial context
        properties.insert(initial_context_var.clone());

        Contract {
            transition_methods: vec![],
            properties,
            contexts: HashMap::new(),
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

    pub fn contexts(&self) -> &HashMap<usize, Rc<Variable>> {
        &self.contexts
    }

    pub fn add_context(&mut self, context_id: usize, context: Rc<Variable>) {
        self.contexts.insert(context_id, context);
    }

    pub fn add_method(&mut self, method: Transition<'a>) {
        self.transition_methods.push(method);
    }
}
