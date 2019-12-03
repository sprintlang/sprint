pub mod expression;
pub mod program;
pub mod state;

use self::program::TERMINAL_ID;
use crate::{
    jog::{contract::Contract, expression::Expression, variable::Variable},
    numbers::Numbers,
};
use sprint_parser::ast;
use std::{collections::HashMap, mem, rc::Rc};

struct Context<'a, 'b> {
    contract: Contract<'a>,
    definitions: HashMap<&'a str, Rc<&'b ast::Definition<'a>>>,
    argument_stack: Vec<Expression<'a>>,
    numbers: Numbers,
    function_context: Option<FunctionContext<'a>>,
    stub_context: Option<StubContext<'a>>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn new(program: impl Iterator<Item = Rc<&'b ast::Definition<'a>>>) -> Self {
        Self {
            contract: Default::default(),
            definitions: program.map(|d| (d.variable.name, d)).collect(),
            argument_stack: Default::default(),
            numbers: Numbers::from(TERMINAL_ID + 1),
            function_context: Default::default(),
            stub_context: Default::default(),
        }
    }
}

impl<'a> Context<'a, '_> {
    fn next_id(&mut self) -> usize {
        match &mut self.function_context {
            Some(context) => &mut context.numbers,
            None => &mut self.numbers,
        }
        .next()
        .unwrap()
    }

    fn push_argument(&mut self, argument: Expression<'a>) {
        self.argument_stack.push(argument);
    }

    fn set_argument_stack(&mut self, stack: Vec<Expression<'a>>) {
        self.argument_stack = stack;
    }

    fn take_argument_stack(&mut self) -> Vec<Expression<'a>> {
        mem::replace(&mut self.argument_stack, Vec::new())
    }
}

#[derive(Debug)]
struct FunctionContext<'a> {
    arguments: Vec<Variable<'a>>,
    name: &'a str,
    numbers: Numbers,
}

impl<'a> FunctionContext<'a> {
    fn new(arguments: Vec<Variable<'a>>, name: &'a str) -> Self {
        Self {
            arguments,
            name,
            numbers: Numbers::from(TERMINAL_ID + 1),
        }
    }
}

struct StubContext<'a> {
    arguments: Vec<Expression<'a>>,
    name: &'a str,
    numbers: Numbers,
    abstracts: HashMap<usize, usize>,
}

impl<'a> StubContext<'a> {
    fn new(context: &mut Context<'a, '_>, definition: &ast::Definition<'a>) -> Self {
        let arguments = context.take_argument_stack().into_iter().rev().collect();
        let mut abstracts = HashMap::new();

        // Handle lookups for the terminal ID sensibly.
        abstracts.insert(TERMINAL_ID, TERMINAL_ID);

        Self {
            arguments,
            name: definition.variable.name,
            numbers: Numbers::from(TERMINAL_ID + 1),
            abstracts,
        }
    }

    fn next_id(&mut self) -> usize {
        self.numbers.next().unwrap()
    }
}
