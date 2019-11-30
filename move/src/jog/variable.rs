use super::{identifier::Identifier, kind::Kind};
use std::fmt::{self, Display, Formatter};

pub const CONTEXTS: Variable<'static> = Variable::new(Identifier::Raw("contexts"), Kind::Contexts);
pub const CONTEXT_REF: Variable<'static> =
    Variable::new(Identifier::Raw("context_ref"), Kind::Context);
pub const CONTRACT_REF: Variable<'static> =
    Variable::new(Identifier::Raw("contract_ref"), Kind::Contract);
pub const OWNER: Variable<'static> = Variable::new(Identifier::Raw("owner"), Kind::Address);

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Variable<'a> {
    identifier: Identifier<'a>,
    kind: Kind,
}

impl<'a> Variable<'a> {
    pub const fn new(identifier: Identifier<'a>, kind: Kind) -> Self {
        Self { identifier, kind }
    }

    pub fn identifier(&self) -> &Identifier<'a> {
        &self.identifier
    }
}

impl Display for Variable<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.identifier, self.kind)
    }
}
