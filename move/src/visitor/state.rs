use super::{definition::TERMINAL_ID, expression, Context};
use crate::{
    jog::{
        action::{
            assert::Assert,
            assign::Assign,
            flip::Flip,
            libra::Withdraw,
            scale::Scale,
            spawn::{PushContext, Spawn},
            update_state::UpdateState,
        },
        expression::{Address, Expression},
        identifier::Identifier,
        kind::Kind,
        method::Method,
        variable::Variable,
    },
    numbers::Numbers,
};
use sprint_parser::ast;
use std::{convert::TryInto, rc::Rc};

pub fn visit<'a>(context: &mut Context<'a>, state: &ast::state::State<'a>) -> usize {
    let key = state as *const _;

    if let Some(&state_id) = context.ids().get(&key) {
        // Do not generate code for the same state twice!
        return state_id;
    }

    // Zero is reserved for the terminal state.
    let id = context.insert(key);

    for transition in state.transitions() {
        let next_id = match transition.next() {
            Some(next) => expression::visit(context, next).try_into().unwrap(),
            None => TERMINAL_ID,
        };

        let mut method = if let Some(args) = context.arguments() {
            let mut method = Method::private(Identifier::Transition(id, next_id));

            for arg in args {
                method.add_argument(arg.clone());
            }

            method.add_argument(Variable::new(Identifier::Raw("context_ref"), Kind::Context));
            method.add_argument(Variable::new(
                Identifier::Raw("origin_state"),
                Kind::Unsigned,
            ));

            method.add_action(Assert::new(
                Expression::Expression("*(&mut copy(context_ref).state) == origin_state".into()),
                1,
            ));

            method
        } else {
            let mut method = Method::public(Identifier::Transition(id, next_id));

            method.add_argument(Variable::new(
                Identifier::Raw("context_index"),
                Kind::Unsigned,
            ));

            method.add_action(Assign::new(
                Variable::new(Identifier::Raw("contexts"), Kind::Contexts),
                Expression::Expression("&mut copy(contract_ref).contexts".into()),
            ));

            method.add_action(Assign::new(
                Variable::new(Identifier::Raw("context_ref"), Kind::Context),
                Expression::Expression(
                    "Vector.borrow_mut<Self.Context>(copy(contexts), copy(context_index))".into(),
                ),
            ));

            method.add_action(Assert::new(
                Expression::Expression(
                    format!(
                        "*(&mut copy(context_ref).state) == {}",
                        id
                    )
                    .into(),
                ),
                1,
            ));

            method
        };

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
                ast::state::Effect::Spawn(root_state) => {
                    let root = expression::visit(context, root_state);

                    let spawned_context = Rc::new(Variable::new(
                        Identifier::Raw("spawned_context"),
                        Kind::Context,
                    ));

                    if context.arguments().is_some() {
                        let variable = Variable::new(
                            Identifier::Spawn(spawn_numbers.next().unwrap()),
                            Kind::Unsigned,
                        );

                        method.add_action(Spawn::new(
                            spawned_context.clone(),
                            Expression::Identifier(variable.identifier().clone()),
                        ));
                        method.add_argument(variable);
                    } else {
                        method.add_action(Spawn::new(spawned_context.clone(), root));
                    }

                    post_actions.push(PushContext::new(spawned_context));
                }
                ast::state::Effect::Withdraw => method.add_action(Withdraw::new(Address::Holder)),
            }
        }

        method.add_action(UpdateState::new(next_id));

        for action in post_actions {
            method.add_action(action);
        }

        context.contract.add_method(method);
    }

    id
}
