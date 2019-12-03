mod jog;
mod numbers;
mod visitor;

use self::visitor::program;
use sprint_parser::ast;

pub fn generate(program: &[ast::Definition]) -> String {
    program::visit(program).to_string()
}
