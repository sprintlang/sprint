use super::{expression::Expression, identifier::Identifier};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Application<'a> {
    arguments: Vec<Expression<'a>>,
    name: Identifier<'a>,
}

impl<'a> From<Identifier<'a>> for Application<'a> {
    fn from(name: Identifier<'a>) -> Self {
        Self {
            arguments: Default::default(),
            name,
        }
    }
}

impl<'a> Application<'a> {
    pub fn add_argument(&mut self, expression: Expression<'a>) {
        self.arguments.push(expression);
    }
}

impl Display for Application<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}(&mut copy(contract_ref)", self.name)?;

        for argument in self.arguments.iter() {
            write!(f, ", {}", argument)?;
        }

        write!(f, ")")
    }
}
