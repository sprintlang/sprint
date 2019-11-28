use super::{definition::TERMINAL_ID, expression, Context};
use crate::{
    jog::{
        action::{
            self,
            assert::Assert,
            assign::Assign,
            flip::Flip,
            libra::Withdraw,
            scale::Scale,
            spawn::{PushContext, Spawn},
            update_state::UpdateState,
        },
        application::Application,
        expression::{Address, Expression},
        identifier::Identifier,
        kind::Kind,
        method::Method,
        variable::{Variable, CONTEXTS, CONTEXT_REF},
    },
    numbers::Numbers,
};
use sprint_parser::ast;
use std::{convert::TryInto, rc::Rc};

pub(super) fn visit<'a>(context: &mut Context<'a>, state: &ast::state::State<'a>) -> usize {
    match &mut context.stub_context {
        Some(_) => visit_stub(context, state),
        None => visit_full(context, state),
    }
}

fn visit_full<'a>(context: &mut Context<'a>, state: &ast::state::State<'a>) -> usize {
    // Zero is reserved for the terminal state.
    let id = context.next_id();

    for transition in state.transitions() {
        let mut from_state = None;
        let (next_id, mut to_state) = match transition.next() {
            Some(next) => match expression::visit(context, next).try_into() {
                Ok(id) => (id, None),
                Err(expression) => (TERMINAL_ID, Some(expression)),
            },
            None => (TERMINAL_ID, None),
        };

        let mut method = match &context.function_context {
            Some(context) => {
                Method::private(Identifier::AbstractTransition(context.name, id, next_id))
            }
            None => Method::public(Identifier::Transition(id, next_id)),
        };

        if let Some(function_context) = &context.function_context {
            let from_variable = Variable::new(Identifier::Raw("from_state"), Kind::Unsigned);
            let to_variable = Variable::new(Identifier::Raw("to_state"), Kind::Unsigned);

            method.add_argument(CONTEXT_REF.clone());

            for arg in &function_context.arguments {
                method.add_argument(arg.clone());
            }

            from_state.replace(from_variable.identifier().clone().into());
            method.add_argument(from_variable);

            to_state.get_or_insert_with(|| {
                let state = to_variable.identifier().clone().into();
                method.add_argument(to_variable);
                state
            });
        } else {
            let context_index = Variable::new(Identifier::Raw("context_index"), Kind::Unsigned);

            method.add_action(Assign::new(
                CONTEXTS.clone(),
                Expression::Expression("&mut copy(contract_ref).contexts".into()),
            ));

            method.add_action(Assign::new(
                CONTEXT_REF.clone(),
                Expression::Expression(
                    format!(
                        "Vector.borrow_mut<context>(copy({}), copy({}))",
                        CONTEXTS.identifier(),
                        context_index.identifier()
                    )
                    .into(),
                ),
            ));

            method.add_argument(context_index);
        }

        method.add_action(Assert::new(
            Expression::Expression(
                format!(
                    "*(&mut copy({}).state) == {}",
                    CONTEXT_REF.identifier(),
                    from_state.unwrap_or(Expression::Unsigned(id))
                )
                .into(),
            ),
            1,
        ));

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

                    if context.function_context.is_some() {
                        let variable = Variable::new(
                            Identifier::Spawn(spawn_numbers.next().unwrap()),
                            Kind::Unsigned,
                        );

                        method.add_action(Spawn::new(
                            spawned_context.clone(),
                            variable.identifier().clone().into(),
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

        method.add_action(UpdateState::new(
            to_state.unwrap_or(Expression::Unsigned(next_id)),
        ));

        for action in post_actions {
            method.add_action(action);
        }

        context.contract.add_method(method);
    }

    id
}

fn visit_stub<'a>(context: &mut Context<'a>, state: &ast::state::State<'a>) -> usize {
    let id = context.next_id();

    let stub_context = context.stub_context.as_mut().unwrap();
    let abstract_id = stub_context.next_id();

    stub_context.abstracts.insert(id, abstract_id);

    for transition in state.transitions() {
        let mut from_state = None;
        let (next_id, mut to_state) = match transition.next() {
            Some(next) => match expression::visit(context, next).try_into() {
                Ok(id) => (id, None),
                Err(expression) => (TERMINAL_ID, Some(expression)),
            },
            None => (TERMINAL_ID, None),
        };

        let stub_context = context.stub_context.as_mut().unwrap();
        let next_abstract_id = *stub_context.abstracts.get(&next_id).unwrap();

        let mut method = match &context.function_context {
            Some(context) => {
                Method::private(Identifier::AbstractTransition(context.name, id, next_id))
            }
            None => Method::public(Identifier::Transition(id, next_id)),
        };

        if let Some(function_context) = &context.function_context {
            let from_variable = Variable::new(Identifier::Raw("from_state"), Kind::Unsigned);
            let to_variable = Variable::new(Identifier::Raw("to_state"), Kind::Unsigned);

            method.add_argument(CONTEXT_REF.clone());

            for arg in &function_context.arguments {
                method.add_argument(arg.clone());
            }

            from_state.replace(from_variable.identifier().clone().into());
            method.add_argument(from_variable);

            to_state.get_or_insert_with(|| {
                let state = to_variable.identifier().clone().into();
                method.add_argument(to_variable);
                state
            });
        } else {
            let context_index = Variable::new(Identifier::Raw("context_index"), Kind::Unsigned);

            method.add_action(Assign::new(
                CONTEXTS.clone(),
                Expression::Expression("&mut copy(contract_ref).contexts".into()),
            ));

            method.add_action(Assign::new(
                CONTEXT_REF.clone(),
                Expression::Expression(
                    format!(
                        "Vector.borrow_mut<context>(copy({}), copy({}))",
                        CONTEXTS.identifier(),
                        context_index.identifier()
                    )
                    .into(),
                ),
            ));

            method.add_argument(context_index);
        }

        let stub_context = context.stub_context.as_mut().unwrap();
        let mut application = Application::from(Identifier::AbstractTransition(
            stub_context.name,
            abstract_id,
            next_abstract_id,
        ));

        application.add_argument(CONTEXT_REF.identifier().clone().into());

        for argument in &stub_context.arguments {
            application.add_argument(argument.clone());
        }

        application.add_argument(from_state.unwrap_or(Expression::Unsigned(id)));

        if to_state.is_none() {
            application.add_argument(Expression::Unsigned(id));
        }

        let mut spawn_numbers = Numbers::default();

        for effect in transition.effects() {
            if let ast::state::Effect::Spawn(root_state) = effect {
                let root = expression::visit(context, root_state);

                if context.function_context.is_some() {
                    let spawn_argument = Identifier::Spawn(spawn_numbers.next().unwrap());
                    method.add_argument(Variable::new(spawn_argument.clone(), Kind::Unsigned));
                    application.add_argument(spawn_argument.into());
                } else {
                    application.add_argument(root);
                }
            }
        }

        method.add_action(action::application::Application::from(application));
        context.contract.add_method(method);
    }

    id
}
