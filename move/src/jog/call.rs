use super::{expression::Expression, identifier::Identifier};
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Call<'a> {
    name: Identifier<'a>,
    arguments: Vec<Expression<'a>>,
}

impl<'a> From<Identifier<'a>> for Call<'a> {
    fn from(name: Identifier<'a>) -> Self {
        Call {
            name,
            arguments: Default::default(),
        }
    }
}

impl<'a> Call<'a> {
    pub fn add_argument(&mut self, expression: Expression<'a>) {
        self.arguments.push(expression);
    }
}

impl Display for Call<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}(&mut copy(contract_ref)", self.name)?;

        for argument in self.arguments.iter() {
            write!(f, ", {}", argument)?;
        }

        write!(f, ")")
    }
}
