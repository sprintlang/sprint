use super::{
    super::{expression::Address, identifier::Identifier, kind::Kind, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

static FLIP_STORE: Variable<'static> = Variable::new(Identifier::Raw("flip_store"), Kind::Address);

#[derive(Default, Debug)]
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
        writeln!(f, "{} = *(&{});", FLIP_STORE.identifier(), Address::Party)?;
        writeln!(
            f,
            "*(&mut {}) = *(&{});",
            Address::Party,
            Address::Counterparty,
        )?;
        write!(
            f,
            "*(&mut {}) = move({});",
            Address::Counterparty,
            FLIP_STORE.identifier()
        )
    }
}
