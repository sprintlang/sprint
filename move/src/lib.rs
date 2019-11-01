mod jog;
mod visitor;

use self::visitor::Contract;
use sprint_parser::ast::state::State;

pub fn generate(state: &State) -> String {
    let mut visitor = Contract::default();
    visitor.visit(state);
    visitor.contract().to_string()
}
