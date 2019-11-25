use super::Command;
use client::client_proxy::ClientProxy;
use sprint_move::script::MoveState;
use std::env;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

pub struct TransitionCommand {}

impl Command for TransitionCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["transition", "t"]
    }

    fn get_params_help(&self) -> &'static str {
        "<sender_account_address>|<sender_account_ref_id> <author_account_address>|<author_account_ref_id> <owner_account_address>|<owner_account_ref_id> <module_name> <context_id> <from_state> <to_state>"
    }

    fn get_description(&self) -> &'static str {
        "Transition to a new state in the contract."
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 8 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        let sender = params[1];

        // Update working directory to where libra repository is found
        // TODO: Remove once libra doesn't rely on compiler cargo memeber
        let current_working_directory = env::current_dir().unwrap();
        let libra_directory = Path::new("../libra");
        assert!(env::set_current_dir(&libra_directory).is_ok());

        println!("Generating transaction code...");
        let move_state = MoveState {
            author: params[2].into(),
            module: params[4].into(),
            owner: params[3].into(),
            context_id: params[5].parse().unwrap(),
            from_state: params[6].parse().unwrap(), // TODO: Make this implicit, fetch it from context
            to_state: params[7].parse().unwrap(),
        };

        // Create a file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", move_state).ok();
        let file_path = file.path().to_str().unwrap();
        println!("Sucessfully generated transaction code!");

        // Compile move program
        println!("Compiling generated move program...");

        let compiled_path;
        match client.compile_program(&["", sender, file_path, "script"]) {
            Ok(path) => {
                println!("Successfully compiled transition script!");
                compiled_path = path;
            }
            Err(e) => {
                println!("Failed to compile transition script to bytecode... {}", e);
                return;
            }
        };

        // Deploy byte code
        println!("Publishing transaction...");

        match client.publish_module(&[params[0], sender, &compiled_path]) {
            Ok(_) => println!("Successfully published transcation"),
            Err(e) => {
                println!("Failed to publish transaction... {}", e);
                return;
            }
        }

        // Change working directory back to original working directory.
        // TODO: Remove once libra doesn't rely on compiler cargo memeber
        assert!(env::set_current_dir(&current_working_directory).is_ok());
    }
}
