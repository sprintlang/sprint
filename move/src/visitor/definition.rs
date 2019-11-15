use super::expression;
use crate::{
    jog::{abstraction::Abstraction, module::Contract},
    numbers::Numbers,
};
use sprint_parser::ast;
use std::collections::HashMap;

pub(super) const TERMINAL_ID: usize = 0;

#[allow(dead_code)]
pub fn visit<'a>(definitions: HashMap<&str, ast::Definition<'a>>) -> Contract<'a> {
    let mut contract = Contract::default();
    let mut numbers = Numbers::from(TERMINAL_ID + 1);

    for (_, definition) in definitions.iter() {
        let mut abstraction = Abstraction::new(definition.name);
        let mut expression = &definition.expression;

        while let ast::Expression::Abstraction(a, e) = expression {
            abstraction.add_argument(a.clone());
            expression = e.as_ref();
        }

        abstraction.set_expression(expression::visit(expression, &mut numbers).into());
        contract.add_abstraction(abstraction);
    }

    contract
}
