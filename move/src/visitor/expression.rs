use super::{state, Context, StubContext};
use crate::jog::{
    call::Call,
    expression::{Address, Expression},
    identifier::Identifier,
};
use sprint_parser::ast;
use std::rc::Rc;

pub(super) fn visit<'a>(
    context: &mut Context<'a, '_>,
    expression: &ast::Expression<'a>,
) -> Expression<'a> {
    match expression {
        ast::Expression {
            expression: ast::ExpressionType::Abstraction(_, r),
            ..
        } => visit(context, r),
        ast::Expression {
            expression: ast::ExpressionType::Application(f, a),
            ..
        } => visit_application(context, f, a),
        ast::Expression {
            expression: ast::ExpressionType::Boolean(_),
            ..
        } => unimplemented!(),
        ast::Expression {
            expression: ast::ExpressionType::Class(c),
            ..
        } => visit_class(context, c),
        ast::Expression {
            expression: ast::ExpressionType::Observable(o),
            ..
        } => visit_observable(context, o),
        ast::Expression {
            expression: ast::ExpressionType::State(s),
            ..
        } => visit_state(context, s),
        ast::Expression {
            expression: ast::ExpressionType::Variable(v),
            ..
        } => visit_variable(context, v),
        ast::Expression {
            expression: ast::ExpressionType::Word(w),
            ..
        } => Expression::Expression(w.to_string().into()),
    }
}

fn visit_application<'a>(
    context: &mut Context<'a, '_>,
    abstraction: &ast::Expression<'a>,
    argument: &ast::Expression<'a>,
) -> Expression<'a> {
    let stack = context.take_argument_stack();
    let argument = visit(context, argument);
    context.set_argument_stack(stack);

    context.push_argument(argument);
    visit(context, abstraction)
}

fn visit_class<'a>(_context: &mut Context<'a, '_>, class: &ast::Class<'a>) -> Expression<'a> {
    match class {
        ast::Class::Comparable(_) => unimplemented!(),
        ast::Class::Equatable(_) => unimplemented!(),
        ast::Class::Negatable(_) => unimplemented!(),
        ast::Class::Numerable(_) => unimplemented!(),
    }
}

fn visit_observable<'a>(
    context: &mut Context<'a, '_>,
    observable: &ast::Observable<'a>,
) -> Expression<'a> {
    match observable {
        ast::Observable::IsParty => {
            Expression::Expression(format!("get_txn_address() == {}", Address::Party).into())
        }
        ast::Observable::IsCounterparty => {
            Expression::Expression(format!("get_txn_address() == {}", Address::Counterparty).into())
        }
        ast::Observable::Konst(e) => visit(context, e),
    }
}

fn visit_state<'a>(context: &mut Context<'a, '_>, state: &ast::state::State<'a>) -> Expression<'a> {
    Expression::Unsigned(state::visit(context, state))
}

fn visit_variable<'a>(
    context: &mut Context<'a, '_>,
    variable: &ast::Variable<'a>,
) -> Expression<'a> {
    match context.definitions.get(variable.name) {
        None => Identifier::Prefixed(variable.name).into(),
        Some(definition) => {
            let definition = definition.clone();

            if results_in_state(variable.kind.clone()) {
                if context.stub_context.is_none() {
                    context.stub_context = Some(StubContext::new(context, &definition));
                    let expression = visit(context, &definition.expression);
                    context.stub_context = None;

                    expression
                } else {
                    context.take_argument_stack();
                    visit(context, &definition.expression)
                }
            } else {
                let mut call = Call::from(Identifier::Prefixed(variable.name));

                for argument in context.take_argument_stack().into_iter().rev() {
                    call.add_argument(argument);
                }

                call.into()
            }
        }
    }
}

fn results_in_state(kind: Rc<ast::Kind>) -> bool {
    match kind.as_ref() {
        ast::Kind::Abstraction(_, s) => results_in_state(s.clone()),
        ast::Kind::State => true,
        ast::Kind::Unresolved(k) => k.borrow().clone().map_or(false, results_in_state),
        _ => false,
    }
}
