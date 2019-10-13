use std::collections::HashSet;
use askama::Template;

#[derive(Template)]
#[template(path = "contract.mvir.template", escape = "none")]
pub struct ContractModule {
    name: String,
    pub is_conditional: bool,
    pub dependencies: HashSet<String>,
    // (name, type, initial_value)
    pub contract_items: Vec<VariableWithDefaultValue>,

    pub create_method: Method,
    pub acquire_method: Method,
}

impl ContractModule {
    pub fn new(name: String) -> Self {
        ContractModule {
            name: name,
            is_conditional: false,
            dependencies: HashSet::new(),
            contract_items: Vec::new(),
            create_method: Method::default(),
            acquire_method: Method::default(),
        }
    }
}

pub struct VariableWithDefaultValue {
    pub var: Variable,
    pub default: String,
}

pub struct Variable {
    pub name: String,
    pub type_name: String,
}

pub struct Method {
    pub params: Vec<Variable>,
    pub var_defs: Vec<Variable>,
    pub actions: Vec<String>,
}

impl Method {
    pub fn default() -> Self {
        Method {
            params: Vec::new(),
            var_defs: Vec::new(),
            actions: Vec::new(),
        }
    }
}
