mod commands;

use self::commands::{Command, DeployCommand};
use chrono::prelude::{SecondsFormat, Utc};
use client::{client_proxy::ClientProxy, commands::*};
use rustyline::{config::CompletionType, error::ReadlineError, Config, Editor};
use std::{collections::HashMap, sync::Arc};

fn main() -> std::io::Result<()> {
    let (commands, alias_to_cmd) = get_commands();

    let mut client_proxy = ClientProxy::new(
        "localhost",
        5001,
        "./swarm_server_files/consensus_peers.config.toml",
        "./swarm_server_files/temp_faucet_keys",
        true, // sync_on_wallet_recovery
        None, // args.faucet_server,
        None, // args.mnemonic_file,
    )
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, &format!("{}", e)[..]))?;

    // TODO: Figure out why this fails... INVALID SIGANTURE
    // let test_ret = client_proxy.test_validator_connection();
    // if let Err(e) = test_ret {
    //     println!("Not able to connect to validator, error {:?}", e);
    //     return Ok(());
    // }

    // let cli_info = format!("Connected to validator");
    // print_help(&cli_info, &commands);

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .auto_add_history(true)
        .build();
    let mut rl = Editor::<()>::with_config(config);

    // Creates an Account
    match client_proxy.create_next_account(true) {
        Ok(account_data) => println!(
            "Created/retrieved account #{} address {}",
            account_data.index,
            hex::encode(account_data.address)
        ),
        Err(e) => report_error("Error creating account", e),
    }

    // Command input loop
    loop {
        let readline = rl.readline("sprint > ");
        match readline {
            Ok(line) => {
                let params = parse_cmd(&line);
                if params.is_empty() {
                    continue;
                }
                match alias_to_cmd.get(&params[0]) {
                    Some(cmd) => {
                        println!("{}", Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true));
                        cmd.execute(&mut client_proxy, &params);
                    }
                    None => match params[0] {
                        "quit" | "q!" => break,
                        "help" | "h" => print_help(&commands),
                        "" => continue,
                        x => println!("Unknown command: {:?}", x),
                    },
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}

type Commands = (
    Vec<Arc<dyn Command>>,
    HashMap<&'static str, Arc<dyn Command>>,
);

/// Returns all the commands available, as well as the reverse index from the aliases to the
/// commands.
fn get_commands() -> Commands {
    let commands: Vec<Arc<dyn Command>> = vec![
        // Arc::new(AccountCommand {}),
        // Arc::new(QueryCommand {}),
        // Arc::new(TransferCommand {}),
        Arc::new(DeployCommand {}),
    ];

    let mut alias_to_cmd = HashMap::new();
    for command in &commands {
        for alias in command.get_aliases() {
            alias_to_cmd.insert(alias, Arc::clone(command));
        }
    }
    (commands, alias_to_cmd)
}

/// Print the help message for the client and underlying command.
fn print_help(commands: &[std::sync::Arc<dyn Command>]) {
    println!("usage: <command> <args>\n\nUse the following commands:\n");
    for cmd in commands {
        println!(
            "{} {}\n\t{}",
            cmd.get_aliases().join(" | "),
            cmd.get_params_help(),
            cmd.get_description()
        );
    }

    println!("help | h \n\tPrints this help");
    println!("quit | q! \n\tExit this client");
    println!("\n");
}
