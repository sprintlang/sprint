use sprint_parser::ast::contract::Contract;
use crate::move_visitor::generate_move_code;

pub mod jog;
mod move_visitor;

pub fn ast_to_move_code(contract: &Contract)-> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    generate_move_code(contract, &mut buffer);

    buffer
}
