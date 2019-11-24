use super::Command;
use client::client_proxy::ClientProxy;
use sprintc::compile;

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

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 3 {
            println!("Invalid number of arguments");
            println!("Usage: deploy <sender_account_address>|<sender_account_ref_id> <file_path>");
            return;
        }

        // match compile() {

        // }

        let compiled_path;
        println!(">> Compiling program");
        match client.compile_program(&[params[0], params[1], params[2], "module"]) {
            Ok(path) => {
                println!("Successfully compiled a program at {}", path);
                compiled_path = path;
            }
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        match client.publish_module(&[params[0], params[1], &compiled_path]) {
            Ok(_) => println!("Successfully published module"),
            Err(e) => {
                println!("{}", e);
                return;
            }
        }
    }
}
