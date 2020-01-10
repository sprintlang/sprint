mod account;
mod deploy;
mod deposit;
mod events;
mod initialize;
mod transition;

pub use self::account::AccountCommand;
pub use self::deploy::DeployCommand;
pub use self::deposit::DepositCommand;
pub use self::events::EventsCommand;
pub use self::initialize::InitializeCommand;
pub use self::transition::TransitionCommand;

use client::client_proxy::ClientProxy;
use std::collections::HashMap;
use std::env;
use std::path::Path;

/// Trait to perform client operations.
pub trait Command {
    /// all commands and aliases this command support.
    fn get_aliases(&self) -> Vec<&'static str>;
    /// string that describes params.
    fn get_params_help(&self) -> &'static str {
        ""
    }
    /// string that describes what the command does.
    fn get_description(&self) -> &'static str;
    /// code to execute.
    fn execute(&self, client: &mut ClientProxy, params: &[&str]);
}

enum PublishType {
    Module,
    Script,
}

impl PublishType {
    pub fn to_str(&self) -> String {
        match self {
            PublishType::Module => String::from("module"),
            PublishType::Script => String::from("script"),
        }
    }
}

fn publish(
    client: &mut ClientProxy,
    sender: &str,
    move_code_path: &str,
    publish_type: PublishType,
) {
    // Update working directory to where libra repository is found
    // TODO: Remove once libra doesn't rely on compiler cargo memeber
    let current_working_directory = env::current_dir().unwrap();
    // TODO: Unhard code this
    let libra_directory = Path::new("../libra");
    assert!(env::set_current_dir(&libra_directory).is_ok());

    // Compile move program
    println!("Compiling generated move program...");

    let compiled_path;
    match client.compile_program(&["", sender, move_code_path, &publish_type.to_str()]) {
        Ok(path) => {
            println!("Successfully compiled move code to bytecode!");
            compiled_path = path;
        }
        Err(e) => {
            println!("Failed to compile move code to bytecode... {}", e);
            return;
        }
    };

    match publish_type {
        PublishType::Module => {
            // Deploy byte code
            println!("Publishing program...");

            match client.publish_module(&["", sender, &compiled_path]) {
                Ok(_) => println!("Successfully published module"),
                Err(e) => {
                    println!("[ERROR]: Failed to publish... {}", e);
                    return;
                }
            }
        }
        PublishType::Script => {
            // Deploy byte code
            println!("Executing script...");

            match client.execute_script(&["", sender, &compiled_path]) {
                Ok(_) => println!("Successfully executed!"),
                Err(e) => println!("[ERROR]: Failed to execute script... {}", e),
            }
        }
    }

    // Change working directory back to original working directory.
    // TODO: Remove once libra doesn't rely on compiler cargo memeber
    assert!(env::set_current_dir(&current_working_directory).is_ok());
}

/// Print the help message for all sub commands.
pub fn print_subcommand_help(parent_command: &str, commands: &[Box<dyn Command>]) {
    println!(
        "usage: {} <arg>\n\nUse the following args for this command:\n",
        parent_command
    );
    for cmd in commands {
        println!(
            "{} {}\n\t{}",
            cmd.get_aliases().join(" | "),
            cmd.get_params_help(),
            cmd.get_description()
        );
    }
    println!("\n");
}

/// Execute sub command.
pub fn subcommand_execute(
    parent_command_name: &str,
    commands: Vec<Box<dyn Command>>,
    client: &mut ClientProxy,
    params: &[&str],
) {
    let mut commands_map = HashMap::new();
    for (i, cmd) in commands.iter().enumerate() {
        for alias in cmd.get_aliases() {
            if commands_map.insert(alias, i) != None {
                panic!("Duplicate alias {}", alias);
            }
        }
    }

    if params.is_empty() {
        print_subcommand_help(parent_command_name, &commands);
        return;
    }

    match commands_map.get(&params[0]) {
        Some(&idx) => commands[idx].execute(client, &params),
        _ => print_subcommand_help(parent_command_name, &commands),
    }
}
