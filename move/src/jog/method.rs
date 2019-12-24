use super::{action::Action, expression::Expression, identifier::Identifier, variable::Variable};
use askama::Template;
use std::collections::HashSet;

#[derive(Template)]
#[template(path = "method.mvir", escape = "none")]
pub struct Method<'a> {
    public: bool,
    identifier: Identifier<'a>,
    arguments: Vec<Variable<'a>>,
    actions: Vec<Box<dyn Action + 'a>>,
    result: Option<Expression<'a>>,
    acquires_resource: bool,
}

impl<'a> Method<'a> {
    fn new(public: bool, identifier: Identifier<'a>) -> Self {
        Method {
            public,
            identifier,
            arguments: Default::default(),
            actions: Default::default(),
            result: Default::default(),
            acquires_resource: false,
        }
    }

    pub fn private(identifier: Identifier<'a>) -> Self {
        Method::new(false, identifier)
    }

    pub fn public(identifier: Identifier<'a>) -> Self {
        Method::new(true, identifier)
    }

    pub fn dependencies(&self) -> Vec<&str> {
        self.actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
            .collect()
    }

    pub fn definitions(&self) -> HashSet<&Variable> {
        self.actions
            .iter()
            .flat_map(|action| action.definitions())
            .collect()
    }

    pub fn add_argument(&mut self, argument: Variable<'a>) {
        self.arguments.push(argument);
    }

    pub fn set_arguments(&mut self, arguments: Vec<Variable<'a>>) {
        assert!(self.arguments.is_empty());
        self.arguments = arguments;
    }

    pub fn add_action(&mut self, action: impl Action + 'a) {
        self.actions.push(Box::new(action));
    }

    pub fn set_result(&mut self, expression: Expression<'a>) {
        self.result = Some(expression);
    }

    pub fn set_acquires_resource(&mut self) {
        self.acquires_resource = true;
    }

    fn result(&self) -> String {
        self.result
            .as_ref()
            .map(|e| format!(" {}", e))
            .unwrap_or_default()
    }
}
