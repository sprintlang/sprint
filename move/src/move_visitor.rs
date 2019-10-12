use sprint_parser::ast::contract;

use crate::jog::contract_module::ContractModule;
use crate::jog::transactions::LockLibra;

pub struct MoveVisitor {
    // base_module: ContractModule,
    modules: Vec<ContractModule>,
    // The module we are currently generating at the stage of visiting we are at.
    curr_module_index: usize,
}

impl MoveVisitor {
    pub fn default() -> Self {
        let base_module = ContractModule::new(String::from("Some cool contract"));

        MoveVisitor {
            // base_module,
            modules: vec![base_module],
            curr_module_index: 0,
        }
    }
}

impl contract::Visitor for MoveVisitor {
    /// The empty contract.
    fn visit_zero(&mut self) {}

    fn visit_one(&mut self) {
        let curr_module = &mut self.modules[self.curr_module_index];

        LockLibra::new(curr_module, 1 /* * multipler*/);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contract::Visitor;

    #[test]
    fn visit_zero() {
        let mut visitor: MoveVisitor = MoveVisitor::default();
        visitor.visit_zero();
        // assert_eq!(visitor.move_code, String::new());
    }

    #[test]
    fn visit_one() {
        let mut visitor: MoveVisitor = MoveVisitor::default();
        visitor.visit_one();
        // assert_eq!(visitor.move_code, MOVE_ONE_CODE);
    }
}
