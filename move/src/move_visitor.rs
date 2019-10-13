use crate::jog::Template as JogTemplate;
use askama::Template as AskamaTemplate;
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
        (*curr_module)
            .create_method
            .actions
            .extend(lock_action.to_string().iter().cloned());
        (*curr_module)
            .acquire_method
            .actions
            .extend(unlock_action.to_string().iter().cloned());
    }
}

/**
 * Templates
 */

impl JogTemplate for ContractModule {
    fn write(&self, w: &mut impl io::Write) {
        let rendered_template = self.render().unwrap();
        w.write_all(rendered_template.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use contract::Visitor;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    #[test]
    fn visit_zero() {
        let mut visitor: MoveVisitor = MoveVisitor::default();
        visitor.visit_zero();

        let module_file = "contracts/generated/zero.generated.mvir";

        let mut buffer = File::create(module_file).unwrap();
        visitor.modules[0].write(&mut buffer);

        test_output(module_file, "contracts/tests/zero.test.mvir");
    }

    #[test]
    fn visit_one() {
        let mut visitor: MoveVisitor = MoveVisitor::default();
        visitor.visit_one();

        let module_file = "contracts/generated/one.generated.mvir";

        let mut buffer = File::create(module_file).unwrap();
        visitor.modules[0].write(&mut buffer);

        test_output(module_file, "contracts/tests/one.test.mvir");
    }

    fn test_output(move_module_file: &str, test_module_file: &str) {
        let mut test_file = File::create("test_file.mvir").unwrap();

        let move_module_file = File::open(move_module_file).unwrap();
        let mut buf_reader = BufReader::new(move_module_file);
        let mut module_contents = String::new();
        buf_reader.read_to_string(&mut module_contents).unwrap();

        let test_module_file = File::open(test_module_file).unwrap();
        let mut buf_reader = BufReader::new(test_module_file);
        let mut test_contents = String::new();
        buf_reader.read_to_string(&mut test_contents).unwrap();

        writeln!(test_file, "//! account: sprint").unwrap();
        writeln!(test_file, "//! account: alice").unwrap();
        writeln!(test_file, "//! account: bob").unwrap();
        writeln!(test_file, "//! account: chris").unwrap();
        writeln!(test_file).unwrap();
        writeln!(test_file, "//! new-transaction").unwrap();
        writeln!(test_file, "//! sender: sprint").unwrap();

        test_file.write_all(module_contents.as_bytes()).unwrap();
        test_file.write_all(test_contents.as_bytes()).unwrap();
    }
}
