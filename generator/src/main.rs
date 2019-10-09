use sprint_parser::ast::contract::Visitor;

struct MoveVisitor {}

static MOVE_ONE_CODE: &str = include!("./sample_move_program.rs");

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
    println!("My Move code is: {}", MOVE_ONE_CODE);
    let mut visitor: MoveVisitor = MoveVisitor::new();
    visitor.visit_one();
}
