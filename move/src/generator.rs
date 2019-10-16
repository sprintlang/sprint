use crate::jog::{
    action::libra::{Lock, Unlock},
    module::contract::Contract,
};
use sprint_parser::ast::{self, contract::Visitor};

pub struct Generator<'a> {
    contract: Contract<'a>,
}

impl<'a> Generator<'a> {
    pub fn new(name: &'a str) -> Self {
        let base_module = Contract::new(name);

        Generator {
            contract: base_module,
        }
    }

    pub fn contract(&self) -> &Contract {
        &self.contract
    }
}

impl Visitor for Generator<'_> {
    fn visit_zero(&mut self) {
        // Default contract is automatically the zero contract.
    }

    fn visit_one(&mut self) {
        let lock_action = Lock::new(1);
        let unlock_action = Unlock::new(&lock_action);

        self.contract.initialize().add_action(lock_action);
        self.contract.acquire().add_action(unlock_action);
    }

    fn visit_give(&mut self, _contract: &ast::contract::Contract) {
        unimplemented!();
    }
}
