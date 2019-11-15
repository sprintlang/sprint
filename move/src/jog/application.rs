use super::expression::Expression;
use std::collections::vec_deque::VecDeque;

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
