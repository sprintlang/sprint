use sprint_parser::ast::contract::Visitor;

static MOVE_ONE_CODE: &str = include_str!("./move_one_contract.mvir");

#[derive(Default)]
pub struct MoveVisitor {
    move_code: String, // Accumulates Move code.
}

impl MoveVisitor {
    pub fn new() -> Self {
        MoveVisitor {
            ..Default::default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit_zero() {
        let mut visitor: MoveVisitor = MoveVisitor::new();
        visitor.visit_zero();
        assert_eq!(visitor.move_code, String::new());
    }

    #[test]
    fn test_visit_one() {
        let mut visitor: MoveVisitor = MoveVisitor::new();
        visitor.visit_one();
        assert_eq!(visitor.move_code, MOVE_ONE_CODE);
    }
}
