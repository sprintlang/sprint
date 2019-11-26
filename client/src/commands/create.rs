use super::{publish, Command, PublishType};
use client::client_proxy::ClientProxy;
use sprint_move::script::CreateContract;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct CreateCommand {}

impl Command for CreateCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["create", "c"]
    }

    fn get_params_help(&self) -> &'static str {
        "<sender_account_address>|<sender_account_ref_id> <author_account_address>|<author_account_ref_id> <module_name> <party> <counterparty>"
    }

    fn get_description(&self) -> &'static str {
        "Create and initalize a new instance of a deployed contract."
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 5 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        let sender = params[1];

        println!("Generating transaction code...");
        // TODO: Add proper error handling if any of these are invalid.
        let author = client
            .get_account_address_from_parameter(params[2])
            .unwrap()
            .short_str();
        let party = client
            .get_account_address_from_parameter(params[4])
            .unwrap()
            .short_str();
        let counterparty = client
            .get_account_address_from_parameter(params[5])
            .unwrap()
            .short_str();

        let move_state = CreateContract {
            author,
            module: params[3].into(),
            party,
            counterparty,
        };

        // Create a file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", move_state).ok();
        let file_path = file.path().to_str().unwrap();
        println!("Sucessfully generated transaction code!");

        println!("Compiling sprint program...");

        let move_code_path = fs::canonicalize(&file_path).unwrap().to_owned();
        let move_code_path = move_code_path.to_str().unwrap();

        publish(client, sender, &move_code_path, PublishType::Script);
    }
}
