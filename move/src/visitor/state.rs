use super::Expression;
use crate::jog::{
    action::{
        flip::Flip,
        libra::{Address, Withdraw},
        scale::Scale,
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
    ids: HashMap<*const ast::State, usize>,
}

impl<'a> State<'a> {
    pub fn visit(&mut self, state: &ast::State) -> usize {
        let key = state as *const _;

        if let Some(&id) = self.ids.get(&key) {
            // Do not generate code for the same state twice!
            return id;
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
                    ast::Effect::Scale(observable) => {
                        let mut visitor = Expression::default();
                        visitor.visit_observable(observable);
                        method.add_action(Scale::new(visitor.expression()));
                    }
                    ast::Effect::Spawn(_state) => {
                        // TODO: implement
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
