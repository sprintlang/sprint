use super::Command;
use client::client_proxy::ClientProxy;
use sprintc::compile;
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
            println!("Usage: deploy <sender_account_address>|<sender_account_ref_id> <file_path>");
            return;
        }

        let sender = params[1];
        let source = params[2];
        let source_path = PathBuf::from(source);

        let move_code_path;
        match compile(&source_path, &None, false) {
            Ok(path) => {
                println!("Successfully compiled {} to move code!", source);
                move_code_path = String::from(path.to_str().unwrap());
            }
            Err(e) => {
                println!("Failed to compiler {} to move code... {}", source, e);
                return;
            }
        }

        let compiled_path;
        match client.compile_program(&[params[0], sender, &move_code_path, "module"]) {
            Ok(path) => {
                println!("Successfully compiled generated move code to bytecode!");
                compiled_path = path;
            }
            Err(e) => {
                println!("Failed to compile generated move code to bytecode... {}", e);
                return;
            }
        };

        match client.publish_module(&[params[0], sender, &compiled_path]) {
            Ok(_) => println!("Successfully published module"),
            Err(e) => {
                println!("Failed to publish module... {}", e);
                return;
            }
        }
    }
}
