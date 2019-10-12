use crate::jog::Template;
use std::collections::HashSet;
use std::io;

pub struct ContractModule {
    _name: String,
    pub is_conditional: bool,
    pub dependencies: HashSet<String>,
    // (name, type, initial_value)
    pub contract_items: Vec<(String, String, String)>,

    pub create_method: Method,
    pub acquire_method: Method,
}

impl ContractModule {
    pub fn new(name: String) -> Self {
        ContractModule {
            _name: name,
            is_conditional: true,
            dependencies: HashSet::new(),
            contract_items: Vec::new(),
            create_method: Method::default(),
            acquire_method: Method::default(),
        }
    }
}

impl Template for ContractModule {
    fn write(&self, _w: &mut impl io::Write) {}
}

pub struct Variable {
    pub name: String,
    pub type_name: String,
}

pub struct Method {
    pub var_defs: Vec<Variable>,
    pub actions: Vec<String>,
}

impl Method {
    pub fn default() -> Self {
        Method {
            var_defs: Vec::new(),
            actions: Vec::new(),
        }
    }
}
