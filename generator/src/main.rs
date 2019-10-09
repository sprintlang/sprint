use sprint_parser::ast::contract::Visitor;

struct MoveVisitor {
    move_code: String, // Accumulates Move code.
}

static MOVE_ONE_CODE: &str = include_str!("./move_one_contract.mvir");

impl MoveVisitor {
    fn new() -> MoveVisitor {
        MoveVisitor {
            move_code: String::new(),
        }
    }
}

impl Visitor for MoveVisitor {
    fn visit_zero(&mut self) {
        // The empty contract.
    }

    fn visit_one(&mut self) {
        self.move_code.push_str(MOVE_ONE_CODE);
    }
}

fn main() {
    let mut visitor: MoveVisitor = MoveVisitor::new();
    visitor.visit_one();
    println!("{}", visitor.move_code);
}
