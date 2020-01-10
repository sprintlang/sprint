use super::Command;
use client::client_proxy::ClientProxy;
use libra_types::language_storage::TypeTag;

pub struct EventsCommand {}

impl Command for EventsCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["events", "e"]
    }

    fn get_params_help(&self) -> &'static str {
        "<account_address>|<account_ref_id>"
    }

    fn get_description(&self) -> &'static str {
        "Get the events emitted to an account!"
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 2 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        let version: u64;

        match client.get_latest_account_state(&params) {
            Ok((_, v)) => version = v,
            Err(e) => {
                println!("Error getting latest account state: {}", e);
                return;
            }
        }

        println!("Version: {}", version);
        println!("Start: {}", &(version - 1000).to_string());

        match client.get_committed_txn_by_range(&[
            "",
            &(version - 1000).to_string(),
            "1000",
            "true",
        ]) {
            Ok(comm_txns_and_events) => {
                let mut count = 0;

                for (_, opt_events) in comm_txns_and_events {
                    if let Some(events) = opt_events {
                        if !events.is_empty() {
                            for event in events {
                                if let TypeTag::Struct(struct_tag) = event.type_tag() {
                                    if struct_tag.name.as_ident_str().as_str() == "Some Name" {
                                        // TODO: Match
                                        count += 1;
                                    }
                                    println!("{:?}", struct_tag);
                                }
                            }
                        }
                    }
                }

                if count == 0 {
                    println!("No events returned");
                }
            }
            Err(e) => println!("Error getting committed transactions by range: {}", e),
        }
    }
}
