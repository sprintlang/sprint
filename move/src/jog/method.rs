use super::{action::Action, expression::Expression, variable::Variable};
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Default)]
pub struct Transition<'a> {
    conditions: Vec<Rc<Condition<'a>>>,
    actions: Vec<Box<dyn Action + 'a>>,
    origin_state: usize,
    to_state: usize,
}

impl<'a> Transition<'a> {
    pub fn new(origin_state: usize, to_state: usize) -> Self {
        Transition {
            conditions: Vec::new(),
            actions: Vec::new(),
            origin_state,
            to_state,
        }
    }

    pub fn dependencies(&self) -> Vec<&str> {
        self.actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
            .collect()
    }

    pub fn properties(&self) -> Vec<Rc<Variable>> {
        self.actions
            .iter()
            .flat_map(|action| action.properties())
            .collect()
    }

    pub fn definitions(&self) -> HashSet<Rc<Variable>> {
        self.actions
            .iter()
            .flat_map(|action| action.definitions())
            .collect()
    }

    pub fn conditions(&self) -> &[Rc<Condition<'a>>] {
        self.conditions.as_slice()
    }

    pub fn add_condition(&mut self, condition: Rc<Condition<'a>>) {
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

    pub fn add_action(&mut self, action: impl Action + 'a) {
        self.actions.push(Box::new(action));
    }
}

pub struct Condition<'a> {
    condition: Expression<'a>,
    error_code: u64,
}

impl<'a> Condition<'a> {
    pub fn new(condition: Expression<'a>, error_code: u64) -> Self {
        Condition {
            condition,
            error_code,
        }
    }
}

impl Display for Condition<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "assert({}, {});", self.condition, self.error_code)
    }
}
