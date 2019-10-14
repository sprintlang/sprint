use crate::jog::Template as JogTemplate;
use askama::Template as AskamaTemplate;
use sprint_parser::ast::contract::{Contract, Visitor as ContractVisitor};
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

impl ContractVisitor for MoveVisitor {
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
        lock_action.init_in_method(&mut (*curr_module).initialize_method);
        unlock_action.init_in_method(&mut (*curr_module).acquire_method);

        // NOTE: If we do what is above we won't need to call .to_string() here anymore.
        (*curr_module)
            .initialize_method
            .actions
            .extend(lock_action.to_string().iter().cloned());
        (*curr_module)
            .acquire_method
            .actions
            .extend(unlock_action.to_string().iter().cloned());
    }

    fn visit_give(&mut self, _contract: &Contract) {}
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
    use colored::*;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::fs::copy;
    use std::fs::create_dir_all;
    use std::fs::remove_file;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;
    use std::process::{Command, Stdio};

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
        let move_module_file = File::open(move_module_file).unwrap();
        let mut buf_reader = BufReader::new(move_module_file);
        let mut module_contents = String::new();
        buf_reader.read_to_string(&mut module_contents).unwrap();

        let test_module_file = File::open(test_module_file).unwrap();
        let mut buf_reader = BufReader::new(test_module_file);
        let mut test_contents = String::new();
        buf_reader.read_to_string(&mut test_contents).unwrap();

        let path_to_libra = "../libra";

        if !Path::new(&format!(
            "{}/language/functional_tests/tests/testsuite",
            path_to_libra
        ))
        .exists()
        {
            panic!("You must clone the libra repository as a subling of the sprint directory.");
        }
        create_dir_all(format!(
            "{}/language/functional_tests/tests/testsuite/sprint/",
            path_to_libra
        ))
        .unwrap();

        let file_name: String = thread_rng().sample_iter(&Alphanumeric).take(15).collect();
        let test_file_path = format!(
            "{}/language/functional_tests/tests/testsuite/sprint/{}.mvir",
            path_to_libra, file_name
        );
        let mut test_file = File::create(test_file_path.clone()).unwrap();

        writeln!(test_file, "//! account: sprint, 1000000").unwrap();
        writeln!(test_file, "//! account: alice, 1000000").unwrap();
        writeln!(test_file, "//! account: bob, 1000000").unwrap();
        writeln!(test_file, "//! account: chris, 1000000").unwrap();
        writeln!(test_file).unwrap();
        writeln!(test_file, "//! new-transaction").unwrap();
        writeln!(test_file, "//! sender: sprint").unwrap();

        test_file.write_all(module_contents.as_bytes()).unwrap();

        writeln!(test_file).unwrap();
        writeln!(test_file).unwrap();

        test_file.write_all(test_contents.as_bytes()).unwrap();

        drop(test_file);

        println!(
            "{}",
            "==================START=OF=LIBRA=TEST=SUITE=OUPUT==================\n"
                .blue()
                .bold()
        );

        let mut cmd = Command::new("cargo")
            .arg("test")
            .arg(format!("--manifest-path={}/Cargo.toml", path_to_libra))
            .arg("-p")
            .arg("functional_tests")
            .arg(format!("sprint/{}", file_name))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        {
            let stdout = cmd.stdout.as_mut().unwrap();
            let stdout_reader = BufReader::new(stdout);
            let stdout_lines = stdout_reader.lines();

            for line in stdout_lines {
                println!("{} {}", ">>".blue().bold(), line.unwrap());
            }

            let stderr = cmd.stderr.as_mut().unwrap();
            let stderr_reader = BufReader::new(stderr);
            let stderr_lines = stderr_reader.lines();

            for line in stderr_lines {
                println!("{} {}", ">>!".red().bold(), line.unwrap());
            }
        }

        let status = cmd.wait().unwrap();

        println!(
            "{}",
            "==================END=OF=LIBRA=TEST=SUITE=OUPUT==================\n"
                .blue()
                .bold()
        );

        if !status.success() {
            copy(test_file_path.clone(), "failed_generated_code.mvir").unwrap();
            println!(
                "{}",
                "Find the failing generated code in failed_generated_code.mvir"
                    .bold()
                    .red()
            );
        }

        remove_file(test_file_path.clone()).unwrap();

        assert!(status.success());
    }
}
