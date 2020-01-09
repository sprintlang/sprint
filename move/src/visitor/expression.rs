use super::{argument, state, Context};
use crate::jog::{
    action::{push::Push, update_state::UpdateState},
    call::Call,
    expression::{Address, Expression},
    identifier::Identifier,
    kind::Kind,
    method::Method,
    variable::{STACK, STACK_LENGTH},
};
use sprint_parser::ast;
use std::{cell::RefCell, rc::Rc};

pub(super) fn visit<'a>(
    context: &mut Context<'a, '_>,
    expression: &ast::Expression<'a>,
) -> Expression<'a> {
    match expression {
        ast::Expression::Abstraction(_, _) => unreachable!("use visit_abstraction instead"),
        ast::Expression::Application(f, a) => visit_application(context, f, a),
        ast::Expression::Boolean(_) => unimplemented!(),
        ast::Expression::Class(c) => visit_class(context, c),
        ast::Expression::Observable(o) => visit_observable(context, o),
        ast::Expression::State(s) => visit_state(context, s),
        ast::Expression::Variable(v) => visit_variable(context, v, Vec::new()),
        ast::Expression::Word(w) => Expression::Expression(w.to_string().into()),
    }
}

pub(super) fn visit_abstraction<'a>(
    context: &mut Context<'a, '_>,
    mut expression: &ast::Expression<'a>,
) -> Expression<'a> {
    while let ast::Expression::Abstraction(_, e) = expression {
        expression = e;
    }

    // Sanity check -- something that doesn't result in a state shouldn't call this function.
    assert!(results_in_state(expression.kind()));

    let key = expression as *const _;

    Expression::State(match context.functions.get(&key) {
        None => {
            let state = Rc::new(RefCell::new(None));
            context.functions.insert(key, state.clone());
            state
        }
        Some(state) => state.clone(),
    })
}

fn visit_application<'a>(
    context: &mut Context<'a, '_>,
    mut abstraction: &ast::Expression<'a>,
    argument: &ast::Expression<'a>,
) -> Expression<'a> {
    let mut arguments = vec![argument];

    while let ast::Expression::Application(e, argument) = abstraction {
        abstraction = e;
        arguments.push(argument);
    }

    match abstraction {
        ast::Expression::Variable(v) => visit_variable(context, v, arguments),
        _ => unreachable!(),
    }
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
            Expression::Expression(format!("get_txn_sender() == *(&{})", Address::Party).into())
        }
        ast::Observable::IsCounterparty => Expression::Expression(
            format!("get_txn_sender() == *(&{})", Address::Counterparty).into(),
        ),
        ast::Observable::Konst(e) => visit(context, e),
    }
}

fn visit_state<'a>(context: &mut Context<'a, '_>, state: &ast::state::State<'a>) -> Expression<'a> {
    Expression::Unsigned(state::visit(context, state))
}

fn visit_variable<'a>(
    context: &mut Context<'a, '_>,
    variable: &ast::Variable<'a>,
    arguments: Vec<&ast::Expression<'a>>,
) -> Expression<'a> {
    match context.definitions.get(variable.name) {
        None => {
            Expression::Copied(Expression::Identifier(Identifier::Prefixed(variable.name)).into())
        }
        Some(definition) => {
            let arguments = arguments.into_iter().rev();
            let definition = definition.clone();

            if results_in_state(variable.kind.clone()) {
                let from = context.numbers.next().unwrap();
                let to = visit_abstraction(context, &definition.expression);

                let function_context = context.function_context.as_ref().unwrap();
                let mut method =
                    Method::transition(function_context.name, &function_context.arguments, from);

                let stacks = arguments.map(|argument| argument::visit(context, argument));

                let mut arguments = Vec::new();
                let mut position = 0;

                for mut pushes in stacks {
                    match pushes.len() {
                        1 => arguments.push(pushes.pop().unwrap()),
                        _ => {
                            position += pushes.len();

                            for push in pushes {
                                method.add_action(push);
                            }

                            arguments.push(Push::with_length(
                                STACK.clone(),
                                Expression::Add(
                                    Expression::Length(
                                        Kind::Unsigned,
                                        Expression::Copied(
                                            Expression::Identifier(STACK.identifier().clone())
                                                .into(),
                                        )
                                        .into(),
                                    )
                                    .into(),
                                    Expression::Unsigned(position - 2).into(),
                                ),
                                STACK_LENGTH.clone(),
                            ));
                        }
                    };
                }

                for argument in arguments {
                    method.add_action(argument);
                }

                method.add_action(UpdateState::new(to));
                context.contract.add_method(method);

                from.into()
            } else {
                let mut call = Call::from(Identifier::Prefixed(variable.name));

                for argument in arguments.map(|argument| visit(context, argument)) {
                    call.add_argument(argument);
                }

                call.into()
            }
        }
    }
}

pub(super) fn results_in_state(kind: Rc<ast::Kind>) -> bool {
    match kind.as_ref() {
        ast::Kind::Abstraction(_, s) => results_in_state(s.clone()),
        ast::Kind::State => true,
        ast::Kind::Unresolved(k) => k.borrow().clone().map_or(false, results_in_state),
        _ => false,
    }
}
