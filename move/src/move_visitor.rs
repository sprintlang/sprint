use crate::jog::Template as JobTemplate;
use askama::Template;
use sprint_parser::ast::contract;
use std::io;

use crate::jog::contract_module::ContractModule;
use crate::jog::transactions::LockLibra;

pub struct MoveVisitor {
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

        let lock_action = LockLibra::on_contract_creation(curr_module, 1 /* * multipler*/);
        (*curr_module).create_method.actions.push(lock_action);
    }
}

impl JobTemplate for ContractModule {
    fn write(&self, _w: &mut impl io::Write) {
        let t = UnconditionalModuleTemplate {
            name: "ContractModuleName",
        }; // instantiate your struct
        println!("{}", t.render().unwrap()); // then render it.
    }
}

#[derive(Template)]
#[template(path = "test.mvir.html")]
struct UnconditionalModuleTemplate<'a> {
    name: &'a str, // the field name should match the variable name
                   // in your template
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
