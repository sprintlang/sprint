use crate::jog::contract_module::ContractModule;
use crate::jog::contract_module::Method;
use crate::jog::contract_module::Variable;
use crate::jog::contract_module::VariableWithDefaultValue;

pub struct LockLibraAction {
    amount: u64,
    /// The name of the variable used to store the Libra in the resource.
    locked_var_name: String,
    /// The name of the variable to temporarily store the Libra being deposited.
    deposit_var_name: String,
}

impl LockLibraAction {
    /// # Arguments
    /// `amount` - The amount of LibraCoins to lock in the contract.
    pub fn new(amount: u64) -> Self {
        // TODO: This might need to be generated to avoid name clashes.
        let locked_var_name = "locked_coins".to_string();
        let deposit_var_name = "deposit_coins".to_string();

        LockLibraAction {
            amount,
            locked_var_name,
            deposit_var_name,
        }
    }

    /// # Arguments
    /// `module` - The module in which to lock the libra's in.
    pub fn init_in_module(&self, module: &mut ContractModule) {
        // Add required imports to the module.
        (*module)
            .dependencies
            .insert(String::from("0x0.LibraAccount"));
        (*module)
            .dependencies
            .insert(String::from("0x0.LibraCoin"));

        // Add item to contract resource to use to store the locked libra.
        (*module).contract_items.push(VariableWithDefaultValue {
            var: Variable {
                name: self.locked_var_name.clone(),
                type_name: "LibraCoin.T".to_string(),
            },
            default: "LibraCoin.zero()".to_string(),
        });
    }

    /// # Arguments
    /// `method` - The method in which we want to execute the libra locking.
    pub fn init_in_method(&self, method: &mut Method) {
        // Add required variable definitions to method.
        (*method).var_defs.push(Variable {
            name: self.locked_var_name.clone(),
            type_name: "LibraCoin.T".to_string(),
        });
        (*method).var_defs.push(Variable {
            name: self.deposit_var_name.clone(),
            type_name: "LibraCoin.T".to_string(),
        });
    }

    pub fn to_string(&self) -> [String; 2] {
        [
            format!("{} = LibraAccnout.withdraw_from_sender({});", self.deposit_var_name, self.amount),
            format!("LibraCoin.deposit(move({}), move({}));", self.locked_var_name, self.deposit_var_name),
        ]
    }
}
