use super::Command;
use client::client_proxy::ClientProxy;

pub struct TransitionCommand {}

impl Command for TransitionCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["transition", "t"]
    }

    fn get_params_help(&self) -> &'static str {
        // TODO: Figure out what params we want
        ""
    }

    fn get_description(&self) -> &'static str {
        "Transition to a new state in the contract."
    }

    #[allow(clippy::needless_return)]
    fn execute(&self, _client: &mut ClientProxy, params: &[&str]) {
        if params.len() != 3 {
            println!("Invalid number of arguments");
            // TODO: Figure out usage
            println!("Usage: transition ...");
            return;
        }
    }
}
