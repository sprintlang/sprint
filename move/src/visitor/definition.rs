use super::Expression;
use sprint_parser::ast;
use std::collections::HashMap;

#[derive(Default)]
pub struct Definition<'a> {
    expressions: HashMap<&'a str, Expression<'a>>,
}

impl<'a> Definition<'a> {
    #[allow(dead_code)]
    pub fn visit(&mut self, definitions: HashMap<&'a str, ast::Definition>) {
        for (name, definition) in definitions.iter() {
            let mut expression = Expression::default();
            expression.visit(&definition.expression);

            self.expressions.insert(name, expression);
        }
    }
}
