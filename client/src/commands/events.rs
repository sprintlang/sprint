use super::Command;
use client::client_proxy::ClientProxy;
use sprint_move::script::MoveState;
use std::env;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

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
        if params.len() != 1 {
            println!("Invalid number of arguments");
            println!("Usage: {} {}", params[0], self.get_params_help());
            return;
        }

        let account = params[1];

        // <account_ref_id>|<account_address> <sent|received> <start_sequence_number> <ascending=true|false> <limit>

        println!(">> Getting events by account and event type.");
        match client.get_events_by_account_and_type(&[
            "", account, "sent",  /*sent|received*/
            "0",     /*start_sequence_number*/
            "false", /*ascending*/
            "100",   /*limit*/
        ]) {
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
            Err(e) => report_error("Error getting events by access path", e),
        }
    }
}
