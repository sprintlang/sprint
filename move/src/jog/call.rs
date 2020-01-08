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
        write!(f, "Self.{}(", self.name)?;

        let mut arguments = self.arguments.iter();

        if let Some(argument) = arguments.next() {
            argument.fmt(f)?;
        }

        for argument in arguments {
            write!(f, ", {}", argument)?;
        }

        write!(f, ")")
    }
}
