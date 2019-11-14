use sprint_parser::ast;
use std::{
    collections::HashMap,
    iter::{self, repeat},
    rc::Rc,
};

type Numbers = iter::Map<iter::Enumerate<iter::Repeat<()>>, fn((usize, ())) -> usize>;

pub struct Abstraction {
    arguments: HashMap<*const ast::Argument, usize>,
    numbers: Numbers,
}

impl Default for Abstraction {
    fn default() -> Self {
        Abstraction {
            arguments: HashMap::default(),
            numbers: repeat(()).enumerate().map(|(i, _)| i),
        }
    }
}

impl Abstraction {
    pub fn add_argument(&mut self, argument: Rc<ast::Argument>) {
        let argument = argument.as_ref() as *const _;
        let i = self.numbers.next().unwrap();

        self.arguments.insert(argument, i);
    }

    #[allow(dead_code)]
    pub fn get_argument(&self, argument: Rc<ast::Argument>) -> Option<usize> {
        let argument = argument.as_ref() as *const _;
        self.arguments.get(&argument).map(Clone::clone)
    }
}
