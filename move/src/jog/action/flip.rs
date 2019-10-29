use crate::jog::{action::Action, variable::Variable};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

const TEMP_FLIP_VAR: &str = "temp_flip_address";

#[derive(Default)]
pub struct Flip();

impl Action for Flip {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![Rc::new(Variable {
            name: TEMP_FLIP_VAR,
            type_name: "address",
            default: None,
        })]
    }
}

impl Display for Flip {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} = *(copy(contract_ref).holder)", TEMP_FLIP_VAR).ok();
        write!(
            f,
            "*(copy(contract_ref).holder) = *(copy(contract_ref).counterparty)"
        )
        .ok();
        write!(f, "*(copy(contract_ref).holder) = {}", TEMP_FLIP_VAR)
    }
}
