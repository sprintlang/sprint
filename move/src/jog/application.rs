use super::expression::Expression;
use std::{
    collections::vec_deque::VecDeque,
    fmt::{self, Display, Formatter},
};

#[derive(Default)]
pub struct Application<'a> {
    arguments: VecDeque<Expression<'a>>,
    name: &'a str,
}

impl<'a> Application<'a> {
    pub fn add_argument(&mut self, expression: Expression<'a>) {
        self.arguments.push_front(expression);
    }

    pub fn set_name(&mut self, name: &'a str) {
        self.name = name;
    }
}

impl Display for Application<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}(&copy(context_ref)", self.name)?;

        for argument in self.arguments.iter() {
            write!(f, ", {}", argument)?;
        }

        write!(f, ")")
    }
}
