mod account;
mod deploy;
mod transition;

pub use self::account::AccountCommand;
pub use self::deploy::DeployCommand;
pub use self::transition::TransitionCommand;
use client::client_proxy::ClientProxy;

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
