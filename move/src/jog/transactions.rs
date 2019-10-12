use crate::jog::contract_module::ContractModule;
use crate::jog::contract_module::Variable;
use crate::jog::Template;
use std::io;

pub struct LockLibra {
    _amount: u64,
    _resource_var: String,
    _coin_var: String,
}

impl LockLibra {
    pub fn on_contract_creation(curr_module: &mut ContractModule, _amount: u64) -> String {
        // Add required imports to current module.
        (*curr_module)
            .dependencies
            .insert(String::from("0x0.LibraAccount"));
        (*curr_module)
            .dependencies
            .insert(String::from("0x0.LibraCoin"));

        let resource_var = String::from("locked_coins");
        let coin_var = String::from("coins");
        let coin_type = String::from("LibraCoin.T");

        // Add item to contract resource to use to store the locked libra.
        (*curr_module).contract_items.push((
            resource_var.clone(),
            coin_type.clone(),
            "LibraCoin.zero()".to_string(),
        ));

        // Add required variable definitions to method.
        (*curr_module).create_method.var_defs.push(Variable {
            name: resource_var.clone(),
            type_name: coin_type.clone(),
        });

        (*curr_module).create_method.var_defs.push(Variable {
            name: coin_var.clone(),
            type_name: coin_type,
        });

        "Lock Libra Action".to_string()

        // For now we just return strings but we will probably want to change this
        // to return a Template to be added to the actions of the create method.
        // LockLibra {
        //     _amount: amount,
        //     _resource_var: resource_var,
        //     _coin_var: coin_var,
        // }
    }
}

impl Template for LockLibra {
    fn write(&self, _w: &mut impl io::Write) {
        // Write:
        // {coin_var} = LibraAccount.withdraw_from_sender(move({amount}));
        // {resource_var} = &mut contract.{resource_var};
        // LibraCoin.deposit(move({resource_var}), {coin_var});
    }
}
