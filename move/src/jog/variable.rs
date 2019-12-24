use super::{identifier::Identifier, kind::Kind};
use lazy_static::lazy_static;
use std::fmt::{self, Display, Formatter};

lazy_static! {
    pub static ref CONTEXTS: Variable<'static> = Variable::new(
        Identifier::Raw("contexts"),
        Kind::MutableReference(Kind::Contexts.into()),
    );
    pub static ref CONTEXT_REF: Variable<'static> = Variable::new(
        Identifier::Raw("context_ref"),
        Kind::MutableReference(Kind::Context.into()),
    );
    pub static ref CONTRACT_REF: Variable<'static> = Variable::new(
        Identifier::Raw("contract_ref"),
        Kind::MutableReference(Kind::Contract.into()),
    );
    pub static ref OWNER: Variable<'static> =
        Variable::new(Identifier::Raw("owner"), Kind::Address);
}

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
