use super::expression;
use crate::{
    jog::{
        action::{
            flip::Flip,
            libra::{Address, Withdraw},
            scale::Scale,
            spawn::{PushContext, Spawn},
            update_state::UpdateState,
        },
        method::{Condition, Transition},
        module,
        variable::Variable,
    },
    numbers::Numbers,
};
use sprint_parser::ast;
use std::{collections::HashMap, rc::Rc};

const TERMINAL_ID: usize = 0;

pub fn visit<'a>(state: &ast::state::State<'a>) -> module::Contract<'a> {
    let mut s = State::default();
    s.visit(state);

    s.into()
}

#[derive(Default)]
struct State<'a> {
    contract: module::Contract<'a>,
    ids: HashMap<*const ast::state::State<'a>, usize>,
    numbers: Numbers,
}

impl<'a> State<'a> {
    pub fn visit(&mut self, state: &ast::state::State<'a>) -> usize {
        let key = state as *const _;

        if let Some(&state_id) = self.ids.get(&key) {
            // Do not generate code for the same state twice!
            return state_id;
        }

        // Zero is reserved for the terminal state.
        let id = self.numbers.next();
        self.ids.insert(key, id);

        for transition in state.transitions() {
            let next_id = match transition.next() {
                Some(next) => match next {
                    ast::Expression::State(s) => self.visit(&s),
                    _ => unreachable!(),
                },
                None => TERMINAL_ID,
            };

            let mut method = Transition::new(id, next_id);

            for condition in transition.conditions() {
                method.add_condition(Condition::new(expression::visit(condition), 0).into());
            }

            let mut post_actions = Vec::new();
            for effect in transition.effects() {
                match effect {
                    ast::state::Effect::Flip => method.add_action(Flip::default()),
                    ast::state::Effect::Scale(scalar) => {
                        method.add_action(Scale::new(expression::visit(scalar)))
                    }
                    ast::state::Effect::Spawn(root_state) => {
                        // TODO: actually visit the state (I guess)
                        let root_state = match root_state {
                            ast::Expression::State(state) => state,
                            _ => unreachable!(),
                        };

                        let root_id = self.visit(root_state);
                        let context = Rc::new(Variable {
                            // TODO: Make this random name gen to allow multiple spawns
                            // in the same transition method
                            name: "spawned_context",
                            type_name: "Self.Context",
                            default: None,
                        });
                        method.add_action(Spawn::new(context.clone(), root_id));
                        post_actions.push(PushContext::new(context));
                    }
                    ast::state::Effect::Withdraw => {
                        method.add_action(Withdraw::new(Address::Holder))
                    }
                }
            }

            method.add_action(UpdateState::new(method.to_state()));
            for action in post_actions {
                method.add_action(action);
            }

            self.contract.add_method(method);
        }

        id
    }
}

impl<'a> From<State<'a>> for module::Contract<'a> {
    fn from(state: State<'a>) -> Self {
        state.contract
    }
}
