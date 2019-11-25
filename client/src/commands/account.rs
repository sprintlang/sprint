use super::Command;
use client::client_proxy::ClientProxy;
use std::collections::HashMap;

/// Major command for account related operations.
pub struct AccountCommand {}

impl Command for AccountCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["account", "a"]
    }
    fn get_description(&self) -> &'static str {
        "Account operations"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(AccountCommandCreate {}),
            Box::new(AccountCommandListAccounts {}),
            Box::new(AccountCommandRecoverWallet {}),
            Box::new(AccountCommandWriteRecovery {}),
            Box::new(AccountCommandMint {}),
        ];

        subcommand_execute(&params[0], commands, client, &params[1..]);
    }
}

/// Sub command to create a random account. The account will not be saved on chain.
pub struct AccountCommandCreate {}

impl Command for AccountCommandCreate {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["create", "c"]
    }
    fn get_description(&self) -> &'static str {
        "Create an account. Returns reference ID to use in other operations"
    }
    fn execute(&self, client: &mut ClientProxy, _params: &[&str]) {
        println!(">> Creating/retrieving next account from wallet");
        match client.create_next_account(true) {
            Ok(account_data) => println!(
                "Created/retrieved account #{} address {}",
                account_data.index,
                hex::encode(account_data.address)
            ),
            Err(e) => println!("[ERROR] Error creating account : {}", e),
        }
    }
}

/// Sub command to recover wallet from the file specified.
pub struct AccountCommandRecoverWallet {}

impl Command for AccountCommandRecoverWallet {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["recover", "r"]
    }
    fn get_params_help(&self) -> &'static str {
        "<file_path>"
    }
    fn get_description(&self) -> &'static str {
        "Recover Libra wallet from the file path"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        println!(">> Recovering Wallet");
        match client.recover_wallet_accounts(&params) {
            Ok(account_data) => {
                println!(
                    "Wallet recovered and the first {} child accounts were derived",
                    account_data.len()
                );
                for data in account_data {
                    println!("#{} address {}", data.index, hex::encode(data.address));
                }
            }
            Err(e) => println!("[ERROR] Error recovering Libra wallet : {}", e),
        }
    }
}

/// Sub command to backup wallet to the file specified.
pub struct AccountCommandWriteRecovery {}

impl Command for AccountCommandWriteRecovery {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["write", "w"]
    }
    fn get_params_help(&self) -> &'static str {
        "<file_path>"
    }
    fn get_description(&self) -> &'static str {
        "Save Libra wallet mnemonic recovery seed to disk"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        println!(">> Saving Libra wallet mnemonic recovery seed to disk");
        match client.write_recovery(&params) {
            Ok(_) => println!("Saved mnemonic seed to disk"),
            Err(e) => println!(
                "[ERROR] Error writing mnemonic recovery seed to file : {}",
                e
            ),
        }
    }
}

/// Sub command to list all accounts information.
pub struct AccountCommandListAccounts {}

impl Command for AccountCommandListAccounts {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["list", "ls"]
    }
    fn get_description(&self) -> &'static str {
        "Print all accounts that were created or loaded"
    }
    fn execute(&self, client: &mut ClientProxy, _params: &[&str]) {
        client.print_all_accounts();
    }
}

/// Sub command to mint account.
pub struct AccountCommandMint {}

impl Command for AccountCommandMint {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["mint", "m"]
    }
    fn get_params_help(&self) -> &'static str {
        "<receiver_account_ref_id>|<receiver_account_address> <number_of_coins>"
    }
    fn get_description(&self) -> &'static str {
        "Mint coins to the account. Suffix 'b' is for blocking"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 3 {
            println!("Invalid number of arguments for mint");
            return;
        }
        println!(">> Minting coins");
        match client.mint_coins(&params, true) {
            Ok(_) => {
                println!("Finished minting!");
            }
            Err(e) => println!("[ERROR] Error minting coins: {}", e),
        }
    }
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
