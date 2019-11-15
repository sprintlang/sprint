mod jog;
mod numbers;
mod visitor;

use self::visitor::definition;
use sprint_parser::ast;
use std::{collections::HashMap, hash::BuildHasher, rc::Rc};

pub fn generate<S: BuildHasher>(definitions: &HashMap<&str, Rc<ast::Definition>, S>) -> String {
    definition::visit(definitions).to_string()
}
