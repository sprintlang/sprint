pub mod definition;
pub mod expression;
pub mod state;

use crate::{
    jog::{contract::Contract, variable::Variable},
    numbers::Numbers,
};
use sprint_parser::ast;
use std::collections::HashMap;

#[derive(Default)]
pub struct Context<'a> {
    pub contract: Contract<'a>,
    arguments: Option<Vec<Variable<'a>>>,
    numbers: Numbers,
    ids: HashMap<*const ast::state::State<'a>, usize>,
}

impl<'a> Context<'a> {
    pub fn insert(&mut self, state: *const ast::state::State<'a>) -> usize {
        let id = self.numbers.next().unwrap();
        self.ids.insert(state, id);

        id
    }

    pub fn set_arguments(&mut self, arguments: Vec<Variable<'a>>) {
        self.arguments = Some(arguments);
    }

    pub fn unset_arguments(&mut self) {
        self.arguments = None;
    }

    pub fn arguments(&self) -> Option<&Vec<Variable<'a>>> {
        self.arguments.as_ref()
    }

    pub fn ids(&self) -> &HashMap<*const ast::state::State<'a>, usize> {
        &self.ids
    }
}
