use super::method::Method;
use askama::Template;
use std::collections::HashSet;

const DEPENDENCIES: [&str; 3] = ["0x0.Vector", "0x0.LibraCoin", "0x0.LibraAccount"];

#[derive(Template, Default)]
#[template(path = "contract.mvir", escape = "none")]
pub struct Contract<'a> {
    initial_state: usize,
    methods: Vec<Method<'a>>,
}

impl<'a> Contract<'a> {
    pub fn dependencies(&self) -> HashSet<&str> {
        let mut dependencies: HashSet<&str> = self
            .methods
            .iter()
            .flat_map(|transition| transition.dependencies())
            .collect();

        for dependency in DEPENDENCIES.iter() {
            dependencies.insert(dependency);
        }

        dependencies
    }

    pub fn add_method(&mut self, method: Method<'a>) {
        self.methods.push(method);
    }

    pub fn set_initial_state(&mut self, state: usize) {
        self.initial_state = state;
    }
}
