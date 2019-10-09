use sprint_parser::ast::contract::Visitor;

struct MoveVisitor {}

static MOVE_ONE_CODE: &str = include_str!("./move_one_contract.mvir");

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
        println!("{}", MOVE_ONE_CODE);
    }
}

fn main() {
    let mut visitor: MoveVisitor = MoveVisitor::new();
    visitor.visit_one();
}
