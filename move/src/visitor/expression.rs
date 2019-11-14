use super::State;
use crate::jog::{
    abstraction::Abstraction, action::libra::Address, application::Application, expression,
};
use sprint_parser::ast;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Default)]
pub struct Expression<'a> {
    expression: expression::Expression<'a>,
}

impl<'a> Expression<'a> {
    pub fn visit(&mut self, expression: &ast::Expression) {
        match expression {
            ast::Expression::Abstraction(a, e) => {
                let abstraction = Abstraction::default();
                self.expression = expression::Expression::Abstraction(abstraction);
                self.visit_abstraction(a.clone(), e);
            }
            _ => self.expression = self.visit_expression(expression),
        };
        self.expression = self.visit_expression(expression);
    }

    pub fn visit_abstraction(
        &mut self,
        argument: Rc<ast::Argument>,
        mut expression: &ast::Expression,
    ) {
        let abstraction = match &self.expression {
            expression::Expression::Abstraction(a) => a,
            _ => unreachable!(),
        };

        abstraction.add_argument(argument);

        while let ast::Expression::Abstraction(a, e) = expression {
            abstraction.add_argument(a.clone());
            expression = e.as_ref();
        }

        abstraction.set_expression(self.visit_expression(expression));
    }

    pub fn visit_expression(&self, expression: &ast::Expression) -> expression::Expression<'a> {
        match expression {
            ast::Expression::Abstraction(_, _) => unreachable!(),
            ast::Expression::Application(f, a) => self.visit_application(f, a),
            ast::Expression::Boolean(_) => unimplemented!(),
            ast::Expression::Class(c) => self.visit_class(c),
            ast::Expression::Observable(o) => self.visit_observable(o),
            ast::Expression::State(s) => self.visit_state(s),
            ast::Expression::Variable(v) => self.visit_variable(v.clone()),
            ast::Expression::Word(w) => expression::Expression::Expression(w.to_string().into()),
        }
    }

    pub fn visit_application(
        &self,
        mut abstraction: &ast::Expression,
        argument: &ast::Expression,
    ) -> expression::Expression<'a> {
        let mut application = Application::default();

        let mut arguments = VecDeque::new();

        arguments.push_front(self.visit_expression(argument));
        while let ast::Expression::Application(f, a) = abstraction {
            arguments.push_front(self.visit_expression(argument));
            abstraction = f;
        }

        for argument in arguments {
            application.add_argument(argument);
        }

        let abstraction = match abstraction {
            ast::Expression::Variable(reference) => match &*reference.borrow() {
                // TODO: actually look up the weak reference
                ast::Variable::Definition(w) => (),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        expression::Expression::default()
    }

    pub fn visit_class(&self, class: &ast::Class) -> expression::Expression<'a> {
        match class {
            ast::Class::Comparable(_) => unimplemented!(),
            ast::Class::Equatable(_) => unimplemented!(),
            ast::Class::Negatable(_) => unimplemented!(),
            ast::Class::Numerable(_) => unimplemented!(),
        }
    }

    pub fn visit_observable(&self, observable: &ast::Observable) -> expression::Expression<'a> {
        match observable {
            ast::Observable::IsHolder => expression::Expression::Expression(
                format!("get_txn_address() == {}", Address::Holder).into(),
            ),
            ast::Observable::IsCounterparty => expression::Expression::Expression(
                format!("get_txn_address() == {}", Address::Counterparty).into(),
            ),
            ast::Observable::Konst(e) => self.visit_expression(e),
        }
    }

    pub fn visit_state(&self, state: &ast::state::State) -> expression::Expression<'a> {
        let mut contract = State::default();
        contract.visit(state);

        expression::Expression::Contract(contract.into())
    }

    pub fn visit_variable(&self, variable: ast::Reference) -> expression::Expression<'a> {
        match &*variable.borrow() {
            ast::Variable::Argument(argument) => {
                let abstraction = match &self.expression {
                    expression::Expression::Abstraction(a) => a,
                    _ => unreachable!(),
                };
                let argument = abstraction.get_argument(argument.clone()).unwrap();

                expression::Expression::Expression(format!("{}", argument).into())
            }
            ast::Variable::Definition(_) => unimplemented!(),
            _ => unreachable!(),
        }
    }

    pub fn expression(self) -> expression::Expression<'a> {
        self.expression
    }
}
