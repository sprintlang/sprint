pub mod definition;
pub mod expression;
pub mod state;

use crate::{
    jog::{contract::Contract, variable::Variable},
    numbers::Numbers,
};
use sprint_parser::ast;
use std::collections::HashMap;

pub struct Context<'a> {
    pub contract: Contract<'a>,
    function_context: Option<FunctionContext<'a>>,
    numbers: Numbers,
    ids: HashMap<*const ast::state::State<'a>, usize>,
}

impl Default for Context<'_> {
    fn default() -> Self {
        Self {
            contract: Default::default(),
            function_context: Default::default(),
            numbers: Numbers::from(1),
            ids: Default::default(),
        }
    }
}

impl<'a> Context<'a> {
    pub fn insert(&mut self, state: *const ast::state::State<'a>) -> usize {
        let id = match &mut self.function_context {
            Some(context) => context.next_id(),
            None => self.numbers.next().unwrap(),
        };
        self.ids.insert(state, id);

        id
    }

    pub fn set_function_context(&mut self, context: FunctionContext<'a>) {
        self.function_context.replace(context);
    }

    pub fn unset_function_context(&mut self) {
        self.function_context.take();
    }

    pub fn function_context(&self) -> Option<&FunctionContext<'a>> {
        self.function_context.as_ref()
    }

    pub fn ids(&self) -> &HashMap<*const ast::state::State<'a>, usize> {
        &self.ids
    }
}

#[derive(Default)]
pub struct FunctionContext<'a> {
    pub arguments: Vec<Variable<'a>>,
    pub name: &'a str,
    numbers: Numbers,
}

impl<'a> FunctionContext<'a> {
    fn new(arguments: Vec<Variable<'a>>, name: &'a str) -> Self {
        Self {
            arguments,
            name,
            numbers: Numbers::default(),
        }
    }

    fn next_id(&mut self) -> usize {
        self.numbers.next().unwrap()
    }
}
