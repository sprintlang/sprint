use super::{expression, Context, FunctionContext};
use crate::jog::{
    contract::Contract, identifier::Identifier, kind::Kind, method::Method, variable::Variable,
};
use sprint_parser::ast;
use std::{convert::TryInto, rc::Rc};

pub(super) const TERMINAL_ID: usize = 0;

#[allow(dead_code)]
pub fn visit<'a>(program: &[ast::Definition<'a>]) -> Contract<'a> {
    let program = program.iter().map(Rc::new);
    let mut context = Context::new(program.clone());

    for definition in program {
        if definition.variable.name == "main" {
            context.function_context.take();
            let state = expression::visit(&mut context, &definition.expression)
                .try_into()
                .unwrap();
            context.contract.set_initial_state(state);
        } else {
            let mut expression = &definition.expression;
            let mut arguments = Vec::new();

            while let ast::Expression::Abstraction(a, e) = expression {
                expression = e;
                arguments.push(Variable::new(Identifier::Prefixed(a.name), Kind::Unsigned));
            }

            if *expression.kind() == ast::Kind::State {
                context
                    .function_context
                    .replace(FunctionContext::new(arguments, definition.variable.name));
                expression::visit(&mut context, expression);
            } else {
                let mut method = Method::private(Identifier::Prefixed(definition.variable.name));

                method.set_arguments(arguments);
                method.set_result(expression::visit(&mut context, expression));
                context.contract.add_method(method);
            }
        }
    }

    context.contract
}
