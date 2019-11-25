mod jog;
mod visitor;

use self::visitor::State;
use sprint_parser::ast::state as ast;

pub use jog::script;

pub fn generate(state: &ast::State) -> String {
    let mut visitor = State::default();
    visitor.visit(state);
    visitor.contract().to_string()
}
