pub mod argument;
pub mod definitions;
pub mod expression;
pub mod state;

use self::definitions::TERMINAL_ID;
use crate::{
    jog::{contract::Contract, method::Method, variable::Variable},
    numbers::Numbers,
};
use sprint_parser::ast;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Context<'a, 'b> {
    contract: Contract<'a>,
    definitions: HashMap<&'a str, Rc<&'b ast::Definition<'a>>>,
    numbers: Rc<RefCell<Numbers>>,
    function_context: Option<FunctionContext<'a>>,
    functions: HashMap<*const ast::Expression<'a>, Rc<RefCell<Option<u64>>>>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(definitions: impl Iterator<Item = Rc<&'b ast::Definition<'a>>>) -> Self {
        Self {
            contract: Default::default(),
            definitions: definitions.map(|d| (d.variable.name, d)).collect(),
            numbers: Rc::new(Numbers::from(TERMINAL_ID + 1).into()),
            function_context: Default::default(),
            functions: Default::default(),
        }
    }
}

#[derive(Debug)]
struct FunctionContext<'a> {
    name: &'a str,
    arguments: Vec<Variable<'a>>,
    method: Option<Method<'a>>,
}

impl<'a> FunctionContext<'a> {
    fn new(name: &'a str, arguments: Vec<Variable<'a>>) -> Self {
        Self {
            name,
            arguments,
            method: Default::default(),
        }
    }

    fn find_argument(&self, name: &'a str) -> Option<u64> {
        self.arguments
            .iter()
            .rev()
            .position(|v| v.identifier().has_name(name))
            .map(|n| n as u64)
    }
}
