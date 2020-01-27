use super::{expression, Context};
use crate::jog::{
    action::push::Push,
    expression::{Binary, Expression},
    kind::Kind,
    variable::STACK,
};
use sprint_parser::ast;

pub(super) fn visit<'a>(
    context: &mut Context<'a, '_>,
    expression: &ast::Expression<'a>,
) -> Vec<Push<'a>> {
    if !expression::results_in_state(expression.kind()) {
        return vec![Push::new(
            STACK.clone(),
            expression::visit(context, expression),
        )];
    }

    match &expression.expression {
        ast::ExpressionType::Application(f, a) => visit_application(context, &f, &a),
        ast::ExpressionType::Variable(v) => vec![Push::new(
            STACK.clone(),
            match context.definitions.get(v.name) {
                Some(definition) => {
                    let definition = definition.clone();
                    expression::visit_abstraction(context, &definition.expression)
                }
                None => expression::visit(context, expression),
            },
        )],
        ast::ExpressionType::State(_) => unimplemented!("state arguments cannot be inlined"),
        _ => unreachable!(),
    }
}

fn visit_application<'a>(
    context: &mut Context<'a, '_>,
    abstraction: &ast::Expression<'a>,
    argument: &ast::Expression<'a>,
) -> Vec<Push<'a>> {
    let mut abstraction = visit(context, abstraction);
    let mut argument = visit(context, argument);

    let mut pushes = Vec::new();
    let mut arguments = Vec::new();

    match abstraction.len() {
        1 => arguments.push(abstraction.pop().unwrap()),
        _ => {
            pushes.append(&mut abstraction);

            arguments.push(Push::new(
                STACK.clone(),
                Expression::Binary(
                    Binary::Subtract,
                    Expression::Length(
                        Kind::Unsigned,
                        Expression::Identifier(STACK.identifier().clone())
                            .copy()
                            .freeze()
                            .into(),
                    )
                    .into(),
                    Expression::Unsigned(
                        2 + match argument.len() {
                            1 => 0,
                            _ => argument.len() as u64,
                        },
                    )
                    .into(),
                ),
            ));
        }
    }

    match argument.len() {
        1 => arguments.push(argument.pop().unwrap()),
        _ => {
            pushes.append(&mut argument);

            arguments.push(Push::new(
                STACK.clone(),
                Expression::Binary(
                    Binary::Subtract,
                    Expression::Length(
                        Kind::Unsigned,
                        Expression::Identifier(STACK.identifier().clone())
                            .copy()
                            .freeze()
                            .into(),
                    )
                    .into(),
                    Expression::Unsigned(3).into(),
                ),
            ));
        }
    }

    pushes.append(&mut arguments);
    pushes
}
