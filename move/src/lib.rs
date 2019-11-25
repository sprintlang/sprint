mod jog;
mod numbers;
mod visitor;

use self::visitor::definitions;
use sprint_parser::ast;

pub use jog::script;

pub fn generate(state: &ast::State) -> String {
    let mut visitor = State::default();
    visitor.visit(state);
    visitor.contract().to_string()
}
