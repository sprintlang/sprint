use crate::generator::generate_move_code;
use sprint_parser::ast::contract::Contract;

mod generator;
pub mod jog;

pub fn ast_to_move_code(contract: &Contract) -> Vec<u8> {
    let mut buffer: Vec<u8> = Vec::new();

    generate_move_code(contract, &mut buffer);

    buffer
}
