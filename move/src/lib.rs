mod jog;
mod numbers;
mod visitor;

use jog::module::Contract;
use sprint_parser::ast;
use std::{collections::HashMap, hash::BuildHasher, rc::Rc};

pub fn generate<S: BuildHasher>(definitions: &HashMap<&str, Rc<ast::Definition>, S>) -> String {
    let mut visitor = visitor::Expression::default();
    visitor.visit(&definitions.get("main").unwrap().expression);

    // TODO: remove this, it is currently here to ensure a contract is returned.
    let mut visitor = visitor::State::default();
    visitor.visit(&ast::state::State::default());

    let contract: Contract = visitor.into();
    contract.to_string()
}
