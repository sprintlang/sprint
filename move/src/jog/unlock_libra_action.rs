use crate::jog::contract_module::ContractModule;
use crate::jog::contract_module::Method;
use crate::jog::lock_libra_action::LockLibraAction;

pub struct UnlockLibraAction {
    /// The name of the variable used to store the Libra in the resource.
    locked_var_name: String,
}

impl UnlockLibraAction {
    /// # Arguments
    /// `amount` - The amount of LibraCoins to lock in the contract.
    pub fn new(lock_libra_action: &LockLibraAction) -> Self {
        UnlockLibraAction {
            locked_var_name: lock_libra_action.locked_var_name.clone(),
        }
    }

    /// # Arguments
    /// `module` - The module in which to lock the libra's in.
    pub fn in_module(self, module: &mut ContractModule) -> Self {
        // Add required imports to the module.
        module.dependencies.insert(String::from("0x0.LibraAccount"));
        module.dependencies.insert(String::from("0x0.LibraCoin"));

        self
    }

    /// # Arguments
    /// `method` - The method in which we want to execute the libra locking.
    pub fn in_method(self, method: &mut Method) -> Self {
        method.actions.extend(self.to_string().iter().cloned());

        self
    }

    fn to_string(&self) -> [String; 1] {
        [format!(
            "LibraAccount.deposit(copy(counterparty), move({}));",
            self.locked_var_name
        )]
    }
}
