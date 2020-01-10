use super::{definitions::TERMINAL_ID, expression, Context};
use crate::{
    jog::{
        action::{
            assert::Assert,
            assign::Assign,
            flip::Flip,
            libra::{DestroyHandle, Emit, Withdraw},
            push::Push,
            scale::Scale,
            spawn::Spawn,
            update_state::UpdateState,
        },
        expression::{Address, Expression},
        identifier::Identifier,
        kind::Kind,
        method::Method,
        variable::{Variable, CONTEXTS, EVENT},
    },
    numbers::Numbers,
};
use sprint_parser::ast;
use std::convert::TryFrom;

pub(super) fn visit<'a>(context: &mut Context<'a, '_>, state: &ast::state::State<'a>) -> u64 {
    if state.is_terminal() {
        return TERMINAL_ID;
    }

    let from = context.numbers.next().unwrap();

    for transition in state.transitions() {
        let to = expression::visit(context, transition.next());

        let function_context = context.function_context.as_ref().unwrap();
        let mut method =
            Method::transition(function_context.name, &function_context.arguments, from);

        for condition in transition.conditions() {
            method.add_action(Assert::new(expression::visit(context, condition), 0));
        }

        let mut post_actions = Vec::new();
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

                    post_actions.push(Push::new(
                        CONTEXTS.clone(),
                        Expression::Moved(
                            Expression::Identifier(spawned_context.identifier().clone()).into(),
                        ),
                    ));
                }
                ast::state::Effect::Withdraw => method.add_action(Withdraw::new(Address::Party)),
            }
        }

        method.add_action(UpdateState::new(to.clone()));

        method.add_action(Assign::new(
            EVENT.clone(),
            Expression::Expression("LibraAccount.new_event_handle<u64>()".into()),
        ));

        // TODO: perform this for all transition methods.
        method.add_action(Emit::new(to));
        method.add_action(DestroyHandle);

        for action in post_actions {
            method.add_action(action);
        }

        context.contract.add_method(method);
    }

    from
}
