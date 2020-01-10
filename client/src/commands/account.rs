use super::{subcommand_execute, Command};
use client::client_proxy::ClientProxy;

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
            Box::new(AccountCommandGetBalance {}),
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

/// Sub commands to query balance for the account specified.
pub struct AccountCommandGetBalance {}

impl Command for AccountCommandGetBalance {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["balance", "b"]
    }
    fn get_params_help(&self) -> &'static str {
        "<account_ref_id>|<account_address>"
    }
    fn get_description(&self) -> &'static str {
        "Get the current balance of an account"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 2 {
            println!("Invalid number of arguments for balance query");
            return;
        }
        match client.get_balance(&params) {
            Ok(balance) => println!("Balance is: {}", balance),
            Err(e) => println!("[ERROE] Failed to get balance: {}", e),
        }
    }
}
