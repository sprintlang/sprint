use super::{expression, Context, FunctionContext};
use crate::jog::{
    contract::Contract, identifier::Identifier, kind::Kind, method::Method, variable::Variable,
};
use sprint_parser::ast;
use std::{convert::TryInto, rc::Rc};

pub(super) const TERMINAL_ID: u64 = 0;

pub fn visit<'a>(definitions: &[ast::Definition<'a>]) -> Contract<'a> {
    let definitions = definitions.iter().map(Rc::new);
    let mut context = Context::new(definitions.clone());

    for definition in definitions {
        let mut expression = &definition.expression;
        let mut arguments = Vec::new();

        while let ast::ExpressionType::Abstraction(a, e) = &expression.expression {
            expression = &e;
            arguments.push(Variable::new(Identifier::Prefixed(a.name), Kind::Unsigned));
        }

        if expression::results_in_state(expression.kind()) {
            context
                .function_context
                .replace(FunctionContext::new(definition.variable.name, arguments));

            let state = expression::visit(&mut context, expression)
                .try_into()
                .unwrap();
            let key = expression as *const _;

            if let Some(s) = context.functions.get(&key) {
                s.borrow_mut().replace(state);
            } else {
                context.functions.insert(key, Rc::new(Some(state).into()));
            }

            if definition.variable.name == "main" {
                context.contract.set_initial_state(state);
            }
        } else {
            let mut method = Method::private(Identifier::Prefixed(definition.variable.name));

            method.set_arguments(arguments);
            method.set_result(expression::visit(&mut context, expression));
            context.contract.add_method(method);
        }
    }

    context
        .contract
        .set_stack_offset(context.numbers.borrow().peek());
    context.contract
}
