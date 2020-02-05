use super::super::{
    action::{drop::DROP_STACK, Action},
    variable::{Variable, TO_STATE},
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct UpdateState;

impl Action for UpdateState {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for UpdateState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        DROP_STACK.fmt(f)?;
        write!(
            f,
            "*(&mut move(context_ref).state) = copy({});",
            TO_STATE.identifier()
        )
    }
}
