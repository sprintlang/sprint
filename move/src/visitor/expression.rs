use super::{state, Context, StubContext};
use crate::jog::{
    call::Call,
    expression::{Address, Expression},
    identifier::Identifier,
};
use sprint_parser::ast;
use std::rc::Rc;

pub(super) fn visit<'a>(
    context: &mut Context<'a>,
    expression: &ast::Expression<'a>,
) -> Expression<'a> {
    match expression {
        ast::Expression::Abstraction(_, r) => visit(context, r),
        ast::Expression::Application(f, a) => visit_application(context, f, a),
        ast::Expression::Boolean(_) => unimplemented!(),
        ast::Expression::Class(c) => visit_class(context, c),
        ast::Expression::Observable(o) => visit_observable(context, o),
        ast::Expression::State(s) => visit_state(context, s),
        ast::Expression::Variable(v) => visit_variable(context, &*v.borrow()),
        ast::Expression::Word(w) => Expression::Expression(w.to_string().into()),
    }
}

fn visit_application<'a>(
    context: &mut Context<'a>,
    abstraction: &ast::Expression<'a>,
    argument: &ast::Expression<'a>,
) -> Expression<'a> {
    let stack = context.take_argument_stack();
    let argument = visit(context, argument);
    context.set_argument_stack(stack);

    context.push_argument(argument);
    visit(context, abstraction)
}

fn visit_class<'a>(_context: &mut Context<'a>, class: &ast::Class<'a>) -> Expression<'a> {
    match class {
        ast::Class::Comparable(_) => unimplemented!(),
        ast::Class::Equatable(_) => unimplemented!(),
        ast::Class::Negatable(_) => unimplemented!(),
        ast::Class::Numerable(_) => unimplemented!(),
    }
}

fn visit_observable<'a>(
    context: &mut Context<'a>,
    observable: &ast::Observable<'a>,
) -> Expression<'a> {
    match observable {
        ast::Observable::IsHolder => {
            Expression::Expression(format!("get_txn_address() == {}", Address::Party).into())
        }
        ast::Observable::IsCounterparty => {
            Expression::Expression(format!("get_txn_address() == {}", Address::Counterparty).into())
        }
        ast::Observable::Konst(e) => visit(context, e),
    }
}

fn visit_state<'a>(context: &mut Context<'a>, state: &ast::state::State<'a>) -> Expression<'a> {
    Expression::Unsigned(state::visit(context, state))
}

fn visit_variable<'a>(context: &mut Context<'a>, variable: &ast::Variable<'a>) -> Expression<'a> {
    match variable {
        ast::Variable::Argument(argument) => Identifier::Prefixed(argument.name).into(),
        ast::Variable::Definition(definition) => {
            let definition = definition.upgrade().unwrap();

            if results_in_state(definition.kind.clone()) {
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
                let mut call = Call::from(Identifier::Prefixed(definition.name));

                for argument in context.take_argument_stack().into_iter().rev() {
                    call.add_argument(argument);
                }

                call.into()
            }
        }
        _ => unreachable!(),
    }
}

fn results_in_state(kind: Rc<ast::Kind>) -> bool {
    match kind.as_ref() {
        ast::Kind::Abstraction(_, s) => results_in_state(s.clone()),
        ast::Kind::State => true,
        ast::Kind::Unresolved(k) => k
            .borrow()
            .clone()
            .map_or(false, |k| results_in_state(k.clone())),
        _ => false,
    }
}
