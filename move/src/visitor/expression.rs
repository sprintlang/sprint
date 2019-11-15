use super::state;
use crate::jog::{
    abstraction::Argument, action::libra::Address, application::Application, expression::Expression,
};
use sprint_parser::ast;

pub fn visit<'a>(expression: &ast::Expression<'a>) -> Expression<'a> {
    match expression {
        ast::Expression::Abstraction(_, _) => unreachable!(),
        ast::Expression::Application(f, a) => visit_application(f, a),
        ast::Expression::Boolean(_) => unimplemented!(),
        ast::Expression::Class(c) => visit_class(c),
        ast::Expression::Observable(o) => visit_observable(o),
        ast::Expression::State(s) => visit_state(s),
        ast::Expression::Variable(v) => visit_variable(v.clone()),
        ast::Expression::Word(w) => Expression::Expression(w.to_string().into()),
    }
}

fn visit_application<'a>(
    mut abstraction: &ast::Expression<'a>,
    argument: &ast::Expression<'a>,
) -> Expression<'a> {
    let mut application = Application::default();

    application.add_argument(visit(argument));
    while let ast::Expression::Application(f, a) = abstraction {
        application.add_argument(visit(a));
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

fn visit_class<'a>(class: &ast::Class<'a>) -> Expression<'a> {
    match class {
        ast::Class::Comparable(_) => unimplemented!(),
        ast::Class::Equatable(_) => unimplemented!(),
        ast::Class::Negatable(_) => unimplemented!(),
        ast::Class::Numerable(_) => unimplemented!(),
    }
}

fn visit_observable<'a>(observable: &ast::Observable<'a>) -> Expression<'a> {
    match observable {
        ast::Observable::IsHolder => {
            Expression::Expression(format!("get_txn_address() == {}", Address::Holder).into())
        }
        ast::Observable::IsCounterparty => {
            Expression::Expression(format!("get_txn_address() == {}", Address::Counterparty).into())
        }
        ast::Observable::Konst(e) => visit(e),
    }
}

fn visit_state<'a>(state: &ast::state::State<'a>) -> Expression<'a> {
    Expression::Contract(state::visit(state))
}

fn visit_variable(variable: ast::Reference) -> Expression<'_> {
    match &*variable.borrow() {
        ast::Variable::Argument(argument) => {
            Expression::Expression(format!("{}", Argument::from(argument.name)).into())
        }
        _ => unreachable!(),
    }
}
