use super::Command;
use client::client_proxy::ClientProxy;
use libra_types::{
    access_path::{AccessPath, Accesses},
    account_address::AccountAddress,
    identifier::Identifier,
    language_storage::StructTag,
};

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

        let account = params[1];

        // <account_ref_id>|<account_address> <sent|received> <start_sequence_number> <ascending=true|false> <limit>

        println!(">> Getting events by account and event type.");

        let account = client.get_account_address_from_parameter(account).unwrap();

        let mut path = AccessPath::resource_access_vec(
            &StructTag {
                address: AccountAddress::default(),
                module: Identifier::new("LibraAccount").unwrap(),
                name: Identifier::new("T").unwrap(),
                type_params: vec![],
            },
            &Accesses::empty(),
        );
        path.extend_from_slice(b"/received_events_count/");
        // path.extend_from_slice(b"/sent_events_count/");
        let access_path = AccessPath::new(account, path.to_vec());

        let start_seq_number = 0;
        let ascending = true;
        let limit = 100;

        match client.client.get_events_by_access_path(
            access_path,
            start_seq_number,
            ascending,
            limit,
        ) {
            Ok((events, last_event_state)) => {
                if events.is_empty() {
                    println!("No events returned");
                } else {
                    for event in events {
                        println!("{}", event);
                    }
                }
                println!("Last event state: {:#?}", last_event_state);
            }
            Err(e) => println!("[ERROR]: Error getting events by access path: {}", e),
        }
    }
}
