use super::method::Method;
use askama::Template;
use std::collections::HashSet;

const DEPENDENCIES: [&str; 3] = ["0x0.Vector", "0x0.LibraCoin", "0x0.LibraAccount"];

#[derive(Template, Default)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    initial_state: u64,
    stack_offset: u64,
    methods: Vec<Method<'a>>,
    dependencies: Vec<&'a str>,
}

impl<'a> Contract<'a> {
    pub fn dependencies(&self) -> HashSet<&str> {
        self.methods
            .iter()
            .flat_map(|method| method.dependencies())
            .chain(DEPENDENCIES.iter().copied())
            .chain(self.dependencies.iter().copied())
            .collect()
    }

    pub fn add_dependency(&mut self, dependency: &'a str) {
        self.dependencies.push(dependency);
    }

    pub fn add_method(&mut self, method: Method<'a>) {
        self.methods.push(method);
    }

    pub fn set_initial_state(&mut self, state: u64) {
        self.initial_state = state;
    }

    pub fn set_stack_offset(&mut self, offset: u64) {
        self.stack_offset = offset;
    }
}
