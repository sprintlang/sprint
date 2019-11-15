use super::{identifier::Identifier, kind::Kind};
use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Clone)]
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
