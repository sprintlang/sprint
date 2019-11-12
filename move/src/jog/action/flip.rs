use super::{super::variable::Variable, libra::Address, Action};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

const TEMP_FLIP_VAR: &str = "temp_flip_address";

#[derive(Default)]
pub struct Flip;

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
        write!(f, "{} = {};", TEMP_FLIP_VAR, Address::Party)?;
        write!(f, "{} = {};", Address::Party, Address::Counterparty,)?;
        write!(f, "{} = {};", Address::Counterparty, TEMP_FLIP_VAR)
    }
}
