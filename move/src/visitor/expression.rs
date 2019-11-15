use super::{state, Context};
use crate::jog::{
    application::Application,
    expression::{Address, Expression},
    identifier::Identifier,
};
use sprint_parser::ast;

pub fn visit<'a>(context: &mut Context<'a>, expression: &ast::Expression<'a>) -> Expression<'a> {
    match expression {
        ast::Expression::Abstraction(_, _) => unreachable!(),
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
    mut abstraction: &ast::Expression<'a>,
    argument: &ast::Expression<'a>,
) -> Expression<'a> {
    let mut application = Application::default();

    application.add_argument(visit(context, argument));
    while let ast::Expression::Application(f, a) = abstraction {
        application.add_argument(visit(context, a));
        abstraction = f;
    }

    match abstraction {
        ast::Expression::Variable(reference) => match &*reference.borrow() {
            ast::Variable::Definition(definition) => {
                let definition = definition.upgrade().unwrap();
                application.set_name(definition.name)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    Expression::Application(application)
}

fn visit_class<'a>(_context: &mut Context, class: &ast::Class<'a>) -> Expression<'a> {
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
            Expression::Expression(format!("get_txn_address() == {}", Address::Holder).into())
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

fn visit_variable<'a>(_context: &mut Context, variable: &ast::Variable<'a>) -> Expression<'a> {
    match variable {
        ast::Variable::Argument(argument) => Identifier::Prefixed(argument.name).into(),
        _ => unreachable!(),
    }
}
