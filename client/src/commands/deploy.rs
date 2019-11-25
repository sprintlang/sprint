use super::Command;
use client::client_proxy::ClientProxy;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct DeployCommand {}

impl Command for DeployCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["deploy", "d"]
    }

    fn get_params_help(&self) -> &'static str {
        "<sender_account_address>|<sender_account_ref_id> <file_path>"
    }

    fn get_description(&self) -> &'static str {
        "Compile and deploy your sprint contract"
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 3 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        let sender = params[1];
        let source = params[2];
        let source_path = PathBuf::from(source);

        // Compile sprint program
        println!("Compiling sprint program...");

        let output_path;
        match sprintc::compile(&source_path, &None, false) {
            Ok(path) => {
                output_path = path;
                // move_code_path = String::from(path.to_str().unwrap());
                println!("Successfully compiled {} to move code!", source);
            }
            Err(e) => {
                println!("Failed to compiler {} to move code... {}", source, e);
                return;
            }
        }

        let move_code_path = fs::canonicalize(&output_path).unwrap().to_owned();
        let move_code_path = move_code_path.to_str().unwrap();

        // Update working directory to where libra repository is found
        // TODO: Remove once libra doesn't rely on compiler cargo memeber
        let current_working_directory = env::current_dir().unwrap();
        let libra_directory = Path::new("../libra");
        assert!(env::set_current_dir(&libra_directory).is_ok());

        // Compile move program
        println!("Compiling generated move program...");

        let compiled_path;
        match client.compile_program(&["", sender, &move_code_path, "module"]) {
            Ok(path) => {
                println!("Successfully compiled generated move code to bytecode!");
                compiled_path = path;
            }
            Err(e) => {
                println!("Failed to compile generated move code to bytecode... {}", e);
                return;
            }
        };

        // Deploy byte code
        println!("Publishing program...");

        match client.publish_module(&[params[0], sender, &compiled_path]) {
            Ok(_) => println!("Successfully published module"),
            Err(e) => {
                println!("Failed to publish module... {}", e);
                return;
            }
        }

        // Change working directory back to original working directory.
        // TODO: Remove once libra doesn't rely on compiler cargo memeber
        assert!(env::set_current_dir(&current_working_directory).is_ok());
    }
}
