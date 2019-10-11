use sprint_parser::ast::contract;

static MOVE_ONE_CODE: &str = include_str!("./move_one_contract.mvir");

#[derive(Default)]
pub struct MoveVisitor {
    /// Accumulates Move code.
    move_code: String,
}

impl contract::Visitor for MoveVisitor {
    /// The empty contract.
    fn visit_zero(&mut self) {}

    fn visit_one(&mut self) {
        self.move_code.push_str(MOVE_ONE_CODE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contract::Visitor;

    #[test]
    fn visit_zero() {
        let mut visitor: MoveVisitor = Default::default();
        visitor.visit_zero();
        assert_eq!(visitor.move_code, String::new());
    }

    #[test]
    fn visit_one() {
        let mut visitor: MoveVisitor = Default::default();
        visitor.visit_one();
        assert_eq!(visitor.move_code, MOVE_ONE_CODE);
    }
}
