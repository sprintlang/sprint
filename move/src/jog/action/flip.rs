use super::{
    super::{expression::Address, identifier::Identifier, kind::Kind, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

static FLIP_STORE: Variable<'static> = Variable::new(Identifier::Raw("flip_store"), Kind::Address);

#[derive(Default)]
pub struct Flip;

impl Action for Flip {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&FLIP_STORE]
    }
}

impl Display for Flip {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} = {};", FLIP_STORE.identifier(), Address::Holder)?;
        write!(f, "{} = {};", Address::Holder, Address::Counterparty,)?;
        write!(
            f,
            "{} = {};",
            Address::Counterparty,
            FLIP_STORE.identifier()
        )
    }
}
