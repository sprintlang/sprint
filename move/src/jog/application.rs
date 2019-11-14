use super::expression::Expression;

#[derive(Default)]
pub struct Application<'a> {
    arguments: Vec<Expression<'a>>,
}

impl<'a> Application<'a> {
    pub fn add_argument(&mut self, expression: Expression<'a>) {
        self.arguments.push(expression);
    }
}
