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

    // #[allow(clippy::needless_return)]
    // fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
    //     if params.len() != 2 {
    //         println!("Invalid number of arguments");
    //         println!("Usage: {} {}", params[0], self.get_params_help());
    //         return;
    //     }

    //     let account = params[1];

    //     // <account_ref_id>|<account_address> <sent|received> <start_sequence_number> <ascending=true|false> <limit>

    //     println!(">> Getting events by account and event type.");

    //     let account = client.get_account_address_from_parameter(account).unwrap();

    //     // -------------

    //     let mut path = AccessPath::resource_access_vec(
    //         &StructTag {
    //             address: AccountAddress::default(),
    //             module: Identifier::new("LibraAccount").unwrap(),
    //             name: Identifier::new("T").unwrap(),
    //             type_params: vec![],
    //         },
    //         &Accesses::empty(),
    //     );
    //     path.extend_from_slice(b"/received_events_count/");

    //     let access_path = AccessPath::new(account, path.to_vec());

    //     println!("Received Events Access Path: {:?}", access_path);

    //     // ------------

    //     let mut path = AccessPath::resource_access_vec(
    //         &StructTag {
    //             address: account, // AccountAddress::default(),
    //             module: Identifier::new("EventTracker").unwrap(),
    //             name: Identifier::new("T").unwrap(),
    //             type_params: vec![],
    //         },
    //         &Accesses::empty(),
    //     );
    //     path.extend_from_slice(b"/my_events_count/");

    //     let access_path = AccessPath::new(account, path.to_vec());

    //     println!("My Events Access Path: {:?}", access_path);

    //     // let access_path = AccessPath::new_for_event(AccountAddress::default(), b"", b"");

    //     // println!("My Events Access Path 2: {:?}", access_path);

    //     // ------------

    //     // let mut path = AccessPath::resource_access_vec(
    //     //     &StructTag {
    //     //         address: AccountAddress::default(),
    //     //         module: Identifier::new("LibraAccount").unwrap(),
    //     //         name: Identifier::new("T").unwrap(),
    //     //         type_params: vec![],
    //     //     },
    //     //     &Accesses::empty(),
    //     // );

    //     // path.extend_from_slice(b"/sent_events_count/");

    //     // let access_path = AccessPath::new(account, path.to_vec());

    //     // println!("Sent Events Access Path: {:?}", access_path);

    //     // ------------

    //     let start_seq_number = 0;
    //     let ascending = true;
    //     let limit = 100;

    //     match client.client.get_events_by_access_path(
    //         access_path,
    //         start_seq_number,
    //         ascending,
    //         limit,
    //     ) {
    //         Ok((events, _last_event_state)) => {
    //             if events.is_empty() {
    //                 println!("No events returned");
    //             } else {
    //                 for event in events {
    //                     println!("{}: {}", event.event_index, event.event);
    //                 }
    //             }
    //             // println!("Last event state: {:#?}", last_event_state);
    //         }
    //         Err(e) => println!("[ERROR]: Error getting events by access path: {}", e),
    //     }
    // }
}
