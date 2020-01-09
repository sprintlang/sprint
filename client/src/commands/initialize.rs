use super::{publish, Command, PublishType};
use askama::Template;
use client::client_proxy::ClientProxy;
use sprint_move::script::InitializeContract;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct InitializeCommand {}

impl Command for InitializeCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["initialize", "i"]
    }

    fn get_params_help(&self) -> &'static str {
        "<author> <module_name> <party> <counterparty>"
    }

    fn get_description(&self) -> &'static str {
        "Initalize a new instance of a deployed contract between two parties."
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 5 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        // TODO: Allow for client to chose the address which executes the transaction
        let sender = "0";

        println!("Generating transaction code...");
        // TODO: Add proper error handling if any of these are invalid.
        let author = hex::encode(
            client
                .get_account_address_from_parameter(params[1])
                .unwrap()
                .to_vec(),
        );
        let party = hex::encode(
            client
                .get_account_address_from_parameter(params[3])
                .unwrap()
                .to_vec(),
        );
        let counterparty = hex::encode(
            client
                .get_account_address_from_parameter(params[4])
                .unwrap()
                .to_vec(),
        );

        let initialize_contract = InitializeContract {
            author: format!("0x{}", author),
            module: params[2].into(),
            party: format!("0x{}", party),
            counterparty: format!("0x{}", counterparty),
        };

        // Create a file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", initialize_contract.render().unwrap()).ok();
        let file_path = file.path().to_str().unwrap();
        println!("Successfully generated transaction code!");

        println!("Compiling sprint program...");

        let move_code_path = fs::canonicalize(&file_path).unwrap();
        let move_code_path = move_code_path.to_str().unwrap();

        let contents =
            fs::read_to_string(move_code_path).expect("Something went wrong reading the file");

        println!("File contents:\n{}", contents);

        publish(client, sender, &move_code_path, PublishType::Script);
    }
}
