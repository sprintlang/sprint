use super::{publish, Command, PublishType};
use askama::Template;
use client::client_proxy::ClientProxy;
use sprint_move::script::CreateContract;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub struct DeployCommand {}

impl DeployCommand {
    fn execute_create_transaction(
        &self,
        client: &mut ClientProxy,
        sender: &str,
        author: &str,
        module: &str,
    ) {
        println!("Generating create transaction code to initialize published module...");

        let create_contract = CreateContract {
            author: format!("0x{}", author),
            module: module.into(),
        };

        // Create a file inside of `std::env::temp_dir()`.
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "{}", create_contract.render().unwrap()).ok();
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

impl Command for DeployCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["deploy", "d"]
    }

    fn get_params_help(&self) -> &'static str {
        "<sender> <file_path> <module_name = Contract>"
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
        let args = sprintc::CompileArgs {
            source_path,
            output_path: None,
            verbose: false,
            check: false,
        };
        match sprintc::compile(&args) {
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

        let move_code_path = fs::canonicalize(&output_path).unwrap();
        let move_code_path = move_code_path.to_str().unwrap();

        publish(client, sender, &move_code_path, PublishType::Module);

        let module_name = "Contract";

        let author = hex::encode(
            client
                .get_account_address_from_parameter(sender)
                .unwrap()
                .to_vec(),
        );

        self.execute_create_transaction(client, sender, &author, module_name);
    }
}
