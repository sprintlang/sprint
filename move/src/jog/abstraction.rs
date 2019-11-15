use super::expression::Expression;
use sprint_parser::ast;
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

pub struct Abstraction<'a> {
    arguments: Vec<Argument<'a>>,
    expression: Box<Expression<'a>>,
    name: &'a str,
}

impl<'a> Abstraction<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            arguments: Default::default(),
            expression: Default::default(),
            name,
        }
    }

    pub fn add_argument(&mut self, argument: Rc<ast::Argument<'a>>) {
        self.arguments.push(argument.name.into());
    }

    pub fn set_expression(&mut self, expression: Box<Expression<'a>>) {
        self.expression = expression;
    }
}

impl Display for Abstraction<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "private fn_{}(context_ref: &Self.Context", self.name)?;

        for argument in self.arguments.iter() {
            write!(f, ", {}: u64", argument)?;
        }

        writeln!(f, "): u64 {{")?;
        writeln!(f, "  return {};", self.expression)?;
        writeln!(f, "}}")
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Argument<'a>(&'a str);

impl<'a> From<&'a str> for Argument<'a> {
    fn from(arg: &'a str) -> Self {
        Self(arg)
    }
}

impl Display for Argument<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "arg_{}", self.0)
    }
}
