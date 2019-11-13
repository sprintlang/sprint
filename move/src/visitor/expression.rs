use super::State;
use crate::jog::{action::libra::Address, expression};
use sprint_parser::ast;

#[derive(Default)]
pub struct Expression<'a> {
    expression: expression::Expression<'a>,
}

impl<'a> Expression<'a> {
    pub fn visit(&mut self, expression: &ast::Expression) {
        match expression {
            ast::Expression::Abstraction(_, _) => unimplemented!(),
            ast::Expression::Application(_, _) => unimplemented!(),
            ast::Expression::Boolean(_) => unimplemented!(),
            ast::Expression::Class(c) => self.visit_class(c),
            ast::Expression::Observable(o) => self.visit_observable(o),
            ast::Expression::State(s) => self.visit_state(s),
            ast::Expression::Variable(_) => unreachable!(),
            ast::Expression::Word(w) => {
                self.expression = expression::Expression::Expression(w.to_string().into())
            }
        };
    }

    pub fn visit_class(&mut self, class: &ast::Class) {
        match class {
            ast::Class::Comparable(_) => unimplemented!(),
            ast::Class::Equatable(_) => unimplemented!(),
            ast::Class::Negatable(_) => unimplemented!(),
            ast::Class::Numerable(_) => unimplemented!(),
        };
    }

    pub fn visit_observable(&mut self, observable: &ast::Observable) {
        match observable {
            ast::Observable::IsHolder => {
                self.expression = expression::Expression::Expression(
                    format!("get_txn_address() == {}", Address::Holder).into(),
                )
            }
            ast::Observable::IsCounterparty => {
                self.expression = expression::Expression::Expression(
                    format!("get_txn_address() == {}", Address::Counterparty).into(),
                )
            }
            ast::Observable::Konst(e) => self.visit(e),
        };
    }

    pub fn visit_state(&mut self, state: &ast::state::State) {
        let mut contract = State::default();
        contract.visit(state);

        self.expression = expression::Expression::Contract(contract.into());
    }

    pub fn expression(self) -> expression::Expression<'a> {
        self.expression
    }
}
