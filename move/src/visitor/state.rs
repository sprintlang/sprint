use super::{definitions::TERMINAL_ID, expression, Context};
use crate::{
    jog::{
        action::{
            assert::Assert, flip::Flip, libra::Withdraw, push::Push, scale::Scale, spawn::Spawn,
        },
        expression::{Address, Expression},
        identifier::Identifier,
        kind::Kind,
        method::Method,
        variable::{Variable, CONTEXTS},
    },
    numbers::Numbers,
};
use sprint_parser::ast;
use std::convert::TryFrom;

pub(super) fn visit<'a>(context: &mut Context<'a, '_>, state: &ast::state::State<'a>) -> u64 {
    if state.is_terminal() {
        return TERMINAL_ID;
    }

    let from = context.numbers.borrow_mut().next().unwrap();

    for transition in state.transitions() {
        let to = expression::visit(context, transition.next());

        let function_context = context.function_context.as_ref().unwrap();
        let mut method = Method::transition(function_context.name, from, to);

        for condition in transition.conditions() {
            method.add_action(Assert::new(expression::visit(context, condition), 0));
        }

        let mut spawn_numbers = Numbers::default();

        for effect in transition.effects() {
            match effect {
                ast::state::Effect::Flip => method.add_action(Flip::default()),
                ast::state::Effect::Scale(scalar) => {
                    method.add_action(Scale::new(expression::visit(context, scalar)))
                }
                ast::state::Effect::Spawn(child_state) => {
                    let child = expression::visit(context, child_state);

                    let spawned_context =
                        Variable::new(Identifier::Raw("spawned_context"), Kind::Context);

                    let expression = match u64::try_from(child) {
                        Ok(_) => {
                            let variable = Variable::new(
                                Identifier::Spawn(spawn_numbers.next().unwrap()),
                                Kind::Unsigned,
                            );
                            let expression = variable.identifier().clone().into();

                            method.add_argument(variable);
                            expression
                        }
                        Err(expression) => expression,
                    };

                    method.add_action(Spawn::new(spawned_context.clone(), expression));

                    method.add_post_action(Push::new(
                        CONTEXTS.clone(),
                        Expression::Identifier(spawned_context.identifier().clone()).r#move(),
                    ));
                }
                ast::state::Effect::Withdraw => method.add_action(Withdraw::new(Address::Party)),
            }
        }

        context.contract.add_method(method);
    }

    from
}
