use crate::{
    ast::{Definition, Variable},
    hash_count::HashCount,
};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Context<'a, T> {
    inner: T,
    pub definitions: HashMap<&'a str, Definition<'a>>,
    pub variables: HashCount<Variable<'a>>,
}

impl<T> From<T> for Context<'_, T> {
    fn from(inner: T) -> Self {
        Self {
            inner,
            definitions: Default::default(),
            variables: Default::default(),
        }
    }
}

impl<'a, T> Context<'a, T> {
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Context<'a, U> {
        Context {
            inner: f(self.inner),
            definitions: self.definitions,
            variables: self.variables,
        }
    }

    pub fn clear(self) -> (Context<'a, ()>, T) {
        (
            Context {
                inner: (),
                definitions: self.definitions,
                variables: self.variables,
            },
            self.inner,
        )
    }
}

impl<T> AsRef<T> for Context<'_, T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
