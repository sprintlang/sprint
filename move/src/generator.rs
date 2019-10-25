use crate::jog::{
    action::libra::{Address, Withdraw},
    method::Transition,
    module::contract::Contract,
};
use sprint_parser::ast::{
    expression::{
        class::{Comparable, Equatable, Numerable},
        kind::{Boolean, Observable, Word},
        visitor::Visitor,
    },
    state::{Effect, State},
};
use std::rc::Rc;

pub struct Generator<'a> {
    contract: Contract<'a>,
    next_id: u64,
}

impl<'a> Generator<'a> {
    pub fn new(name: &'a str) -> Self {
        Generator {
            contract: Contract::new(name),
            next_id: 0,
        }
    }

    pub fn generate(&mut self, state: State) {
        let starting_state = self.next_id();
        self.visit(state.into(), starting_state);
    }

    fn visit(&mut self, state: Rc<State>, state_id: u64) {
        for transition in state.transitions() {
            let to_state_id = self.next_id();
            let mut method = Transition::new(state_id, to_state_id);

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

            match transition.next() {
                Some(next) => self.visit(next, to_state_id),
                None => self.contract.add_terminal_state(to_state_id),
            };
        }
    }

    fn next_id(&mut self) -> u64 {
        let next = self.next_id;
        self.next_id += 1;

        next
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
}

impl<'a> Visitor for ExpressionGenerator<'a> {
    fn visit_boolean(&mut self, _value: &Boolean) {
        unimplemented!();
    }

    fn visit_comparable(&mut self, _value: &Comparable) {
        unimplemented!();
    }

    fn visit_equatable(&mut self, _value: &Equatable) {
        unimplemented!();
    }

    fn visit_numerable(&mut self, _value: &Numerable) {
        unimplemented!();
    }

    fn visit_observable(&mut self, value: &Observable) {
        match value {
            Observable::IsHolder => self.expression.push_str("get_txn_address() == holder"),
            Observable::IsCounterparty => self
                .expression
                .push_str("get_txn_address() == counterparty"),
            Observable::Konst(value) => value.accept(self),
        }
    }

    fn visit_word(&mut self, &Word(value): &Word) {
        self.expression.push_str(&value.to_string());
    }
}
