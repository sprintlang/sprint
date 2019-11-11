use super::{action::Action, expression::Expression, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Default)]
pub struct Transition<'a> {
    conditions: Vec<Rc<Condition>>,
    actions: Vec<Box<dyn Action + 'a>>,
    post_execution_actions: Vec<Box<dyn Action + 'a>>,
    origin_state: usize,
    to_state: usize,
}

impl<'a> Transition<'a> {
    pub fn new(origin_state: usize, to_state: usize) -> Self {
        Transition {
            conditions: Vec::new(),
            actions: Vec::new(),
            post_execution_actions: Vec::new(),
            origin_state,
            to_state,
        }
    }

    pub fn dependencies(&self) -> Vec<&str> {
        let mut dependencies: Vec<&str> = self
            .actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
            .collect();

        let mut post_action_dependencies: Vec<&str> = self
            .post_execution_actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
            .collect();

        dependencies.append(&mut post_action_dependencies);

        dependencies
    }

    pub fn properties(&self) -> Vec<Rc<Variable>> {
        let mut properties: Vec<Rc<Variable>> = self
            .actions
            .iter()
            .flat_map(|action| action.properties())
            .collect();

        let mut post_action_properties: Vec<Rc<Variable>> = self
            .post_execution_actions
            .iter()
            .flat_map(|action| action.properties())
            .collect();

        properties.append(&mut post_action_properties);

        properties
    }

    pub fn definitions(&self) -> Vec<Rc<Variable>> {
        let mut definitions: Vec<Rc<Variable>> = self
            .actions
            .iter()
            .flat_map(|action| action.definitions())
            .collect();

        let mut post_action_definitions: Vec<Rc<Variable>> = self
            .post_execution_actions
            .iter()
            .flat_map(|action| action.definitions())
            .collect();

        definitions.append(&mut post_action_definitions);

        definitions
    }

    pub fn conditions(&self) -> &[Rc<Condition>] {
        self.conditions.as_slice()
    }

    pub fn add_condition(&mut self, condition: Rc<Condition>) {
        self.conditions.push(condition);
    }

    pub fn origin_state(&self) -> usize {
        self.origin_state
    }

    pub fn to_state(&self) -> usize {
        self.to_state
    }

    pub fn actions(&self) -> Vec<&dyn Action> {
        self.actions.iter().map(AsRef::as_ref).collect()
    }

    pub fn post_execution_actions(&self) -> Vec<&dyn Action> {
        self.post_execution_actions
            .iter()
            .map(AsRef::as_ref)
            .collect()
    }

    pub fn add_action(&mut self, action: impl Action + 'a) {
        self.actions.push(Box::new(action));
    }

    pub fn add_post_execution_action(&mut self, action: impl Action + 'a) {
        self.post_execution_actions.push(Box::new(action));
    }
}

pub struct Condition {
    condition: Expression,
    error_code: u64,
}

impl Condition {
    pub fn new(condition: Expression, error_code: u64) -> Self {
        Condition {
            condition,
            error_code,
        }
    }
}

impl Display for Condition {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "assert({}, {});", self.condition, self.error_code)
    }
}
