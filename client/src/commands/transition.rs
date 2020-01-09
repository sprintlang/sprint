use super::{publish, Command, PublishType};
use client::client_proxy::ClientProxy;
use sprint_move::script::Transition;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct TransitionCommand {}

impl Command for TransitionCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["transition", "t"]
    }

    fn get_params_help(&self) -> &'static str {
        "<author> <module_name> <context_id> <function_name...>"
    }

    fn get_description(&self) -> &'static str {
        "Transition to a new state in the contract."
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() < 5 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        let sender = params[1];

        println!("Generating transaction code...");

        let author = hex::encode(
            client
                .get_account_address_from_parameter(params[1])
                .unwrap()
                .to_vec(),
        );

        let move_state = Transition {
            author: format!("0x{}", author),
            module: params[2].into(),
            context_id: params[3].parse().unwrap(),
            function_names: &params[4..params.len()],
        };

        // Create a file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", move_state).ok();
        let file_path = file.path().to_str().unwrap();
        println!("Sucessfully generated transaction code!");

        let move_code_path = fs::canonicalize(&file_path).unwrap();
        let move_code_path = move_code_path.to_str().unwrap();

        let contents =
            fs::read_to_string(move_code_path).expect("Something went wrong reading the file");

        println!("File contents:\n{}", contents);

        publish(client, sender, &move_code_path, PublishType::Script);
    }
}
