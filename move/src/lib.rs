mod jog;
mod numbers;
mod visitor;

use self::visitor::definitions;
use sprint_parser::ast;

pub fn generate(definitions: &[ast::Definition]) -> String {
  definitions::visit(definitions).to_string()
}
