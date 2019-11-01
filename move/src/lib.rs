mod jog;
mod visitor;

use sprint_parser::ast::state::State;

pub fn generate(state: &State) -> String {
    let mut visitor = visitor::State::default();
    visitor.visit(state);
    visitor.contract().to_string()
}
