use super::expression::Expression;
use sprint_parser::ast;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    iter::{self, repeat},
    rc::Rc,
};

type Numbers = iter::Map<iter::Enumerate<iter::Repeat<()>>, fn((usize, ())) -> usize>;

pub struct Abstraction<'a> {
    arguments: HashMap<*const ast::Argument, Argument>,
    expression: Box<Expression<'a>>,
    numbers: Numbers,
}

impl Default for Abstraction<'_> {
    fn default() -> Self {
        Abstraction {
            arguments: HashMap::default(),
            expression: Expression::default().into(),
            numbers: repeat(()).enumerate().map(|(i, _)| i),
        }
    }
}

impl<'a> Abstraction<'a> {
    pub fn add_argument(&mut self, argument: Rc<ast::Argument>) {
        let argument = argument.as_ref() as *const _;
        let i = self.numbers.next().unwrap();

        self.arguments.insert(argument, i.into());
    }

    #[allow(dead_code)]
    pub fn get_argument(&self, argument: Rc<ast::Argument>) -> Option<Argument> {
        let argument = argument.as_ref() as *const _;
        self.arguments.get(&argument).map(Clone::clone)
    }

    pub fn set_expression(&mut self, expression: Expression<'a>) {
        self.expression = expression.into();
    }
}

#[derive(Clone)]
pub struct Argument(usize);

impl From<usize> for Argument {
    fn from(arg: usize) -> Self {
        Self(arg)
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "arg{}", self.0)
    }
}
