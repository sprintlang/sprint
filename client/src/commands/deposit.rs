use super::{publish, Command, PublishType};
use askama::Template;
use client::client_proxy::ClientProxy;
use sprint_move::script::Deposit;
use std::fs;
use std::io::Write;
use tempfile::NamedTempFile;

pub struct DepositCommand {}

impl Command for DepositCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["deposit", "d"]
    }

    fn get_params_help(&self) -> &'static str {
        "<author> <module_name> <depositer> <amount>"
    }

    fn get_description(&self) -> &'static str {
        "Deposit libra into initialized contract."
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 5 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        println!("Generating transaction code...");
        // TODO: Add proper error handling if any of these are invalid.
        let author = hex::encode(
            client
                .get_account_address_from_parameter(params[1])
                .unwrap()
                .to_vec(),
        );

        let deposit = Deposit {
            author: format!("0x{}", author),
            module: params[2].into(),
            amount: params[4].into(),
            coin_store_index: 0, // TODO: Unhard code this
        };

        // Create a file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", deposit.render().unwrap()).ok();
        let file_path = file.path().to_str().unwrap();
        println!("Sucessfully generated transaction code!");

        println!("Compiling sprint program...");

        let move_code_path = fs::canonicalize(&file_path).unwrap();
        let move_code_path = move_code_path.to_str().unwrap();

        let contents =
            fs::read_to_string(move_code_path).expect("Something went wrong reading the file");

        println!("File contents:\n{}", contents);

        publish(client, params[3], &move_code_path, PublishType::Script);
    }
}
