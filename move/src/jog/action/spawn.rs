use super::{
    super::{
        expression::Address,
        expression::Expression,
        variable::{Variable, STACK},
    },
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Spawn<'a> {
    context: Variable<'a>,
    root: Expression<'a>,
}

impl<'a> Spawn<'a> {
    pub fn new(context: Variable<'a>, root: Expression<'a>) -> Self {
        Spawn { context, root }
    }
}

impl Action for Spawn<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&self.context]
    }
}

impl Display for Spawn<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(
            f,
            "{} = Context {{
                state: {},
                coinstore_index: *(&copy(context_ref).coinstore_index),
                party: *(&{}),
                counterparty: *(&{}),
                scale: *(&copy(context_ref).scale),
                stack: Self.clone_stack(freeze(copy({}))),
            }};",
            self.context.identifier(),
            self.root,
            Address::Party,
            Address::Counterparty,
            STACK.identifier(),
        )
    }
}
