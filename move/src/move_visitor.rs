use crate::jog::Template as JogTemplate;
use askama::Template;
use sprint_parser::ast::contract;
use std::io;

use crate::jog::contract_module::ContractModule;
use crate::jog::lock_libra_action::LockLibraAction;
use crate::jog::unlock_libra_action::UnlockLibraAction;

pub struct MoveVisitor {
    modules: Vec<ContractModule>,
    // The module we are currently generating at the stage of visiting we are at.
    curr_module_index: usize,
}

impl MoveVisitor {
    pub fn default() -> Self {
        let base_module = ContractModule::new(String::from("SomeCoolContract"));

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

        let lock_action = LockLibraAction::new(1 /* * multipler*/);
        let unlock_action = UnlockLibraAction::new(&lock_action);

        // NOTE: These init method calls could be moved to be executed once we have visited everything.
        // We could just go through every module and all the methods and init all the actions in them,
        // with the correct modules, and methods. This would probably simplify the code in the visit methods.
        // We could just stored Actions which have the init_in_module and init_in_method methods in the Method.actions Vector.
        lock_action.init_in_module(curr_module);
        unlock_action.init_in_module(curr_module);
        lock_action.init_in_method(&mut (*curr_module).create_method);
        unlock_action.init_in_method(&mut (*curr_module).create_method);

        // NOTE: If we do what is above we won't need to call .to_string() here anymore.
        (*curr_module).create_method.actions.extend(lock_action.to_string().iter().cloned());
        (*curr_module).acquire_method.actions.extend(unlock_action.to_string().iter().cloned());
    }
}

/**
 * Templates
 */

impl JogTemplate for ContractModule {
    fn write(&self, _w: &mut impl io::Write) {
        println!("{}", self.render().unwrap()); // then render it.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contract::Visitor;
    use std::fs::File;

    #[test]
    fn visit_zero() {
        let mut visitor: MoveVisitor = MoveVisitor::default();
        println!();
        println!("-----------------");
        println!("CONTRACT ZERO");
        println!("-----------------");
        visitor.visit_zero();

        let mut buffer = File::create("output.mvir").unwrap();
        visitor.modules[0].write(&mut buffer);

        println!();
    }

    #[test]
    fn visit_one() {
        let mut visitor: MoveVisitor = MoveVisitor::default();
        println!();
        println!("-----------------");
        println!("CONTRACT ONE");
        println!("-----------------");
        visitor.visit_one();

        let mut buffer = File::create("output.mvir").unwrap();
        visitor.modules[0].write(&mut buffer);

        println!();
    }
}
