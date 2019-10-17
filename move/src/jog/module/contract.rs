use super::super::{method::Method, variable::Variable};
use askama::Template;
use std::{collections::HashSet, rc::Rc};

#[derive(Template)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    name: &'a str,
    is_conditional: bool,
    initialize: Method<'a>,
    acquire: Method<'a>,
}

impl<'a> Contract<'a> {
    pub fn new(name: &'a str) -> Self {
        Contract {
            name,
            is_conditional: false,
            initialize: Default::default(),
            acquire: Default::default(),
        }
    }

    pub fn initialize(&mut self) -> &mut Method<'a> {
        &mut self.initialize
    }

    pub fn acquire(&mut self) -> &mut Method<'a> {
        &mut self.acquire
    }

    fn methods(&self) -> impl Iterator<Item = &Method> {
        vec![&self.initialize, &self.acquire].into_iter()
    }

    fn dependencies(&self) -> HashSet<&str> {
        self.methods()
            .flat_map(|method| method.dependencies())
            .collect()
    }

    fn properties(&self) -> HashSet<Rc<Variable>> {
        self.methods()
            .flat_map(|method| method.properties())
            .collect()
    }
}
