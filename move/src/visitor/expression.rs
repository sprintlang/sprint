use crate::jog::expression;
use sprint_parser::ast;

#[derive(Default)]
pub struct Expression {
    expression: expression::Expression,
}

impl<'a> Expression {
    pub fn visit(&mut self, expression: &ast::Expression<'a>) {
        match expression {
            ast::Expression::Application(_, _) => unimplemented!(),
            ast::Expression::Boolean(_) => unimplemented!(),
            ast::Expression::Class(c) => self.visit_class(c),
            ast::Expression::Identifier(_, _) => unimplemented!(),
            ast::Expression::Observable(o) => self.visit_observable(o),
            ast::Expression::Word(w) => self.expression = w.to_string().into(),
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
                self.expression = "get_txn_address() == *(&copy(contract_ref).holder)".into()
            }
            ast::Observable::IsCounterparty => {
                self.expression = "get_txn_address() == *(&copy(contract_ref).counterparty)".into()
            }
            ast::Observable::Konst(e) => self.visit(e),
        };
    }

    pub fn expression(self) -> expression::Expression {
        self.expression
    }
}
