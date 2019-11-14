use super::{super::numbers::Numbers, expression::Expression};
use sprint_parser::ast;
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
    rc::Rc,
    cell::RefCell,
};

#[derive(Default)]
pub struct Abstraction<'a> {
    arguments: RefCell<HashMap<*const ast::Argument, Argument>>,
    expression: Box<RefCell<Expression<'a>>>,
    numbers: Numbers,
}

impl<'a> Abstraction<'a> {
    pub fn add_argument(&self, argument: Rc<ast::Argument>) {
        let argument = argument.as_ref() as *const _;
        let i = self.numbers.next();

        self.arguments.borrow_mut().insert(argument, i.into());
    }

    pub fn get_argument(&self, argument: Rc<ast::Argument>) -> Option<Argument> {
        let argument = argument.as_ref() as *const _;
        self.arguments.borrow().get(&argument).map(Clone::clone)
    }

    pub fn set_expression(&self, expression: Expression<'a>) {
        *(self.expression.borrow_mut()) = expression;
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
