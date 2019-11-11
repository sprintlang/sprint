use super::Expression;
use crate::jog::{
    action::{
        flip::Flip,
        libra::{Address, Withdraw},
        scale::Scale,
        spawn::{Spawn, SpawnSetup},
    },
    method::{Condition, Transition},
    module,
};
use sprint_parser::ast::state as ast;
use std::collections::HashMap;

const TERMINAL_ID: usize = 0;

#[derive(Default)]
pub struct State<'a> {
    contract: module::Contract<'a>,

    // Used for state id generation
    ids: HashMap<*const ast::State<'a>, usize>,
}

impl<'a> State<'a> {
    pub fn visit(&mut self, state: &ast::State<'a>) -> usize {
        let key = state as *const _;

        if let Some(&state_id) = self.ids.get(&key) {
            // Do not generate code for the same state twice!
            return state_id;
        }

        // Zero is reserved for the terminal state.
        let id = self.ids.len() + 1;
        self.ids.insert(key, id);

        for transition in state.transitions() {
            let next_id = match transition.next() {
                Some(next) => self.visit(next.as_ref()),
                None => TERMINAL_ID,
            };

            let mut method = Transition::new(id, next_id);

            for condition in transition.conditions() {
                let mut visitor = Expression::default();
                visitor.visit(condition);
                method.add_condition(Condition::new(visitor.expression(), 0).into());
            }

            for effect in transition.effects() {
                match effect {
                    ast::Effect::Flip => {
                        method.add_action(Flip::default());
                    }
                    ast::Effect::Scale(scalar) => {
                        let mut visitor = Expression::default();
                        visitor.visit(scalar);
                        method.add_action(Scale::new(visitor.expression()));
                    }
                    ast::Effect::Spawn(root_state) => {
                        let root_id = self.visit(root_state);
                        method.add_action(SpawnSetup::default());
                        method.add_post_execution_action(Spawn::new(root_id));
                    }
                    ast::Effect::Withdraw => method.add_action(Withdraw::new(Address::Holder)),
                }
            }

            self.contract.add_method(method);
        }

        id
    }

    pub fn contract(self) -> module::Contract<'a> {
        self.contract
    }
}
