use super::{
    super::{
        expression::Address,
        expression::Expression,
        variable::{Variable, STACK},
    },
    assign::Assign,
    drop::DROP_STACK,
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Spawn<'a> {
    context: Variable<'a>,
    root: Expression<'a>,
    assign: Assign<'a>,
}

impl<'a> Spawn<'a> {
    pub fn new(context: Variable<'a>, root: Expression<'a>) -> Self {
        Spawn {
            context,
            root,
            assign: Assign::new(
                STACK.clone(),
                Expression::Expression("&mut copy(context_ref).stack".into()),
            ),
        }
    }
}

impl Action for Spawn<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        let mut definitions = self.assign.definitions();
        definitions.push(&self.context);
        definitions
    }
}

impl Display for Spawn<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        DROP_STACK.fmt(f)?;
        writeln!(
            f,
            "{} = Context {{
                state: {},
                coinstore_index: *(&copy(context_ref).coinstore_index),
                party: *(&{}),
                counterparty: *(&{}),
                scale: *(&copy(context_ref).scale),
                stack: Self.clone_stack(&copy(context_ref).stack),
            }};",
            self.context.identifier(),
            self.root,
            Address::Party,
            Address::Counterparty,
        )?;
        self.assign.fmt(f)
    }
}
