use super::expression;
use crate::jog::abstraction::Abstraction;
use sprint_parser::ast;
use std::collections::HashMap;

#[derive(Default)]
pub struct Definition<'a> {
    expressions: Vec<Abstraction<'a>>,
}

impl<'a> Definition<'a> {
    #[allow(dead_code)]
    pub fn visit(&mut self, definitions: HashMap<&str, ast::Definition<'a>>) {
        for (_, definition) in definitions.iter() {
            let mut abstraction = Abstraction::new(definition.name);
            let mut expression = &definition.expression;

            while let ast::Expression::Abstraction(a, e) = expression {
                abstraction.add_argument(a.clone());
                expression = e.as_ref();
            }

            abstraction.set_expression(expression::visit(expression).into());
            self.expressions.push(abstraction);
        }
    }
}
