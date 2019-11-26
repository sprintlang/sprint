use super::{publish, Command, PublishType};
use client::client_proxy::ClientProxy;
use std::fs;
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

        publish(client, sender, &move_code_path, PublishType::Module);
    }
}
