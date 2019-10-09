use sprint_parser::ast::contract::Visitor;

struct MoveVisitor {}

impl MoveVisitor {
    fn new() -> MoveVisitor {
        MoveVisitor {}
    }
}

impl Visitor for MoveVisitor {
    fn visit_zero(&mut self) {
        // The empty contract.
    }

    fn visit_one(&mut self) {
        println!("Visting one primitive");
    }
}

fn main() {
    let move_one_code = include!("./sample_move_program.rs");
    println!("My Move code is: {}", move_one_code);
    let mut visitor: MoveVisitor = MoveVisitor::new();
    visitor.visit_one();
}
