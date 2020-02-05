use super::{
    super::{expression::Expression, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Push<'a> {
    vector: Variable<'a>,
    item: Expression<'a>,
}

impl<'a> Push<'a> {
    pub fn new(vector: Variable<'a>, item: Expression<'a>) -> Self {
        Self { vector, item }
    }
}

impl Action for Push<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![&self.vector]
    }
}

impl Display for Push<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // NOTE: does not work with vectors of mutable references.
        write!(
            f,
            "Vector.push_back<{}>(copy({}), {});",
            self.vector.kind().inner(),
            self.vector.identifier(),
            self.item
        )
    }
}
