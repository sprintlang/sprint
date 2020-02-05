use super::{argument, state, Context};
use crate::jog::{
    action::push::Push,
    call::Call,
    expression::{Address, Binary, Expression},
    identifier::Identifier,
    kind::Kind,
    method::Method,
    variable::{STACK, STACK_LENGTH},
};
use chrono::NaiveDate;
use sprint_parser::ast;
use std::{cell::RefCell, rc::Rc};

pub(super) fn visit<'a>(
    context: &mut Context<'a, '_>,
    expression: &ast::Expression<'a>,
) -> Expression<'a> {
    match &expression.expression {
        ast::ExpressionType::Abstraction(_, _) => unreachable!("use visit_abstraction instead"),
        ast::ExpressionType::Application(f, a) => visit_application(context, &f, &a),
        ast::ExpressionType::Boolean(_) => unimplemented!(),
        ast::ExpressionType::Class(c) => visit_class(context, &c),
        ast::ExpressionType::Date(d) => visit_date(context, &d),
        ast::ExpressionType::Observable(o) => visit_observable(context, &o),
        ast::ExpressionType::State(s) => visit_state(context, &s),
        ast::ExpressionType::Variable(v) => visit_variable(context, &v, Vec::new()),
        ast::ExpressionType::Word(w) => Expression::Expression(w.to_string().into()),
    }
}

pub(super) fn visit_abstraction<'a>(
    context: &mut Context<'a, '_>,
    mut expression: &ast::Expression<'a>,
) -> Expression<'a> {
    while let ast::ExpressionType::Abstraction(_, e) = &expression.expression {
        expression = &e;
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

    while let ast::ExpressionType::Application(e, argument) = &abstraction.expression {
        abstraction = &e;
        arguments.push(&argument);
    }

    match &abstraction.expression {
        ast::ExpressionType::Variable(v) => visit_variable(context, &v, arguments),
        _ => unreachable!(),
    }
}

fn visit_class<'a>(context: &mut Context<'a, '_>, class: &ast::Class<'a>) -> Expression<'a> {
    match class {
        ast::Class::Comparable(c) => {
            let (binary, left, right) = match c {
                ast::Comparable::Greater(left, right) => (Binary::Greater, left, right),
                ast::Comparable::Less(left, right) => (Binary::Less, left, right),
                ast::Comparable::GreaterEqual(left, right) => (Binary::GreaterEqual, left, right),
                ast::Comparable::LessEqual(left, right) => (Binary::LessEqual, left, right),
            };

            Expression::Binary(
                binary,
                visit(context, left).into(),
                visit(context, right).into(),
            )
        }
        ast::Class::Equatable(_) => unimplemented!(),
        ast::Class::Negatable(_) => unimplemented!(),
        ast::Class::Numerable(_) => unimplemented!(),
    }
}

fn visit_date<'a>(context: &mut Context<'a, '_>, date: &ast::Date) -> Expression<'a> {
    match date {
        ast::Date::Now => {
            context.contract.add_dependency("{{alice}}.Date");
            Expression::Observable("Date")
        }
        ast::Date::Date(year, month, day, hour, minute, second) => {
            let timestamp = NaiveDate::from_ymd(*year as i32, *month as u32, *day as u32)
                .and_hms(*hour as u32, *minute as u32, *second as u32)
                .timestamp() as u64;
            timestamp.into()
        }
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
        None => match context
            .function_context
            .as_ref()
            .unwrap()
            .find_argument(variable.name)
        {
            None => Expression::Identifier(Identifier::Prefixed(variable.name)).copy(),
            Some(i) => Expression::Get(
                Kind::Unsigned,
                Expression::Identifier(STACK.identifier().clone())
                    .copy()
                    .freeze()
                    .into(),
                Expression::Binary(
                    Binary::Subtract,
                    Expression::Identifier(STACK_LENGTH.identifier().clone())
                        .copy()
                        .into(),
                    Expression::Unsigned(i + 1).into(),
                )
                .into(),
            ),
        },
        Some(definition) => {
            let arguments = arguments.into_iter().rev();
            let definition = definition.clone();

            if results_in_state(variable.kind.clone()) {
                let from = context.numbers.borrow_mut().next().unwrap();
                let to = visit_abstraction(context, &definition.expression);

                let function_context = context.function_context.as_ref().unwrap();
                let mut method = Method::transition(function_context.name, from, to);

                // We need to get context.numbers out before we visit arguments, since until
                // stacks is consumed we can't borrow context immutably.
                let numbers = context.numbers.clone();
                let stacks = arguments.map(|argument| argument::visit(context, argument));

                let mut arguments = Vec::new();
                let mut position = 0;

                for mut pushes in stacks {
                    match pushes.len() {
                        1 => arguments.push(pushes.pop().unwrap()),
                        _ => {
                            position += pushes.len() as u64;

                            for push in pushes {
                                method.add_action(push);
                            }

                            arguments.push(Push::new(
                                STACK.clone(),
                                Expression::Binary(
                                    Binary::Add,
                                    Expression::Numbers(numbers.clone()).into(),
                                    Expression::Binary(
                                        Binary::Add,
                                        Expression::Identifier(STACK_LENGTH.identifier().clone())
                                            .copy()
                                            .into(),
                                        Expression::Unsigned(position - 2).into(),
                                    )
                                    .into(),
                                ),
                            ));
                        }
                    };
                }

                for argument in arguments {
                    method.add_action(argument);
                }

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
