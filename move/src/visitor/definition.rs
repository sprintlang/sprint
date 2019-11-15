use super::{expression, Context};
use crate::jog::{
    contract::Contract, identifier::Identifier, kind::Kind, method::Method, variable::Variable,
};
use sprint_parser::ast;
use std::{collections::HashMap, hash::BuildHasher, rc::Rc};

pub(super) const TERMINAL_ID: usize = 0;

#[allow(dead_code)]
pub fn visit<'a, S: BuildHasher>(
    context: &mut Context<'a>,
    definitions: &HashMap<&str, Rc<ast::Definition<'a>>, S>,
) -> Contract<'a> {
    let mut contract = Contract::default();

    for (_, definition) in definitions.iter() {
        let mut method = Method::new(false, Identifier::Prefixed(definition.name));
        let mut expression = &definition.expression;

        while let ast::Expression {
            expression: ast::ExpressionType::Abstraction(a, e),
            ..
        } = expression
        {
            method.add_argument(Variable::new(Identifier::Prefixed(a.name), Kind::Unsigned));
            expression = e.as_ref();
        }

        method.set_result(expression::visit(context, expression));
        contract.add_method(method);
    }

    contract
}
