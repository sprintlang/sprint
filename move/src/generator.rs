use crate::jog::{
    action::libra::{Address, Withdraw},
    method::Transition,
    module::contract::Contract,
};
use sprint_parser::ast::{
    expression::{Class, Expression, Observable},
    state::{Effect, State},
};
use std::collections::HashMap;

const TERMINAL_ID: usize = 0;

pub struct Generator<'a> {
    contract: Contract<'a>,
    ids: HashMap<*const State, usize>,
}

impl<'a> Generator<'a> {
    pub fn new(name: &'a str) -> Self {
        Generator {
            contract: Contract::new(name),
            ids: HashMap::new(),
        }
    }

    pub fn generate(&mut self, state: &State) -> usize {
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
                Some(next) => self.generate(next.as_ref()),
                None => TERMINAL_ID,
            };

            let mut method = Transition::new(id, next_id);

            for _condition in transition.conditions() {
                // TODO: implement
            }

            for effect in transition.effects() {
                match effect {
                    Effect::Flip => {
                        // TODO: implement
                    }
                    Effect::Scale(_amount) => {
                        // TODO: implement
                    }
                    Effect::Withdraw => method.add_action(Withdraw::new(Address::Holder)),
                }
            }

            self.contract.add_method(method);
        }

        id
    }

    pub fn contract(&self) -> &Contract {
        &self.contract
    }
}

pub struct ExpressionGenerator<'a> {
    _transition: &'a Transition<'a>,
    expression: String,
}

impl<'a> ExpressionGenerator<'a> {
    #[allow(dead_code)]
    fn new(transition: &'a Transition) -> Self {
        ExpressionGenerator {
            _transition: transition,
            expression: String::new(),
        }
    }

    #[allow(dead_code)]
    fn generate(&mut self, expression: &Expression) {
        match expression {
            Expression::Boolean(_) => unimplemented!(),

            Expression::Class(class) => match class {
                Class::Comparable(_) => unimplemented!(),
                Class::Equatable(_) => unimplemented!(),
                Class::Negatable(_) => unimplemented!(),
                Class::Numerable(_) => unimplemented!(),
            },

            Expression::Observable(observable) => match observable {
                Observable::IsHolder => self.expression.push_str("get_txn_address() == holder"),
                Observable::IsCounterparty => self
                    .expression
                    .push_str("get_txn_address() == counterparty"),
                Observable::Konst(expression) => self.generate(expression),
            },

            Expression::Word(word) => self.expression.push_str(&word.to_string()),
        };
    }
}
