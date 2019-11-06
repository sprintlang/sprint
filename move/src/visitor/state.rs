use super::Expression;
use crate::jog::{
    action::{
        flip::Flip,
        libra::{Address, Withdraw},
        scale::Scale,
        spawn::Spawn,
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
    context_id: usize,
}

impl<'a> State<'a> {
    pub fn visit(&mut self, state: &ast::State) {
        self.visit_helper(state, "initial_context");
    }

    pub fn visit_helper(&mut self, state: &ast::State, context: &str) -> usize {
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
                Some(next) => self.visit_helper(next.as_ref(), context),
                None => TERMINAL_ID,
            };

            let mut method = Transition::new(id, next_id, String::from(context));

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
                        self.context_id += 1;
                        let context_name = format!("context_{}", self.context_id);
                        let root_id = self.visit_helper(root_state, &context_name);
                        method.add_action(Spawn::new(context_name, root_id));
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
