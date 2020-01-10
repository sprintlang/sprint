use super::{
    super::{
        expression::{Binary, Expression},
        variable::Variable,
    },
    assign::Assign,
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Push<'a> {
    vector: Variable<'a>,
    item: Expression<'a>,
    increment: Option<Assign<'a>>,
}

impl<'a> Push<'a> {
    pub fn new(vector: Variable<'a>, item: Expression<'a>) -> Self {
        Self {
            vector,
            item,
            increment: None,
        }
    }

    pub fn with_length(vector: Variable<'a>, item: Expression<'a>, length: Variable<'a>) -> Self {
        let identifier = length.identifier().clone();
        Self {
            vector,
            item,
            increment: Some(Assign::new(
                length,
                Expression::Binary(
                    Binary::Add,
                    Expression::Copied(Expression::Identifier(identifier).into()).into(),
                    Expression::Unsigned(1).into(),
                ),
            )),
        }
    }
}

impl Action for Push<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        let mut definitions = vec![&self.vector];
        definitions.extend(
            self.increment
                .as_ref()
                .map(Action::definitions)
                .iter()
                .flatten(),
        );
        definitions
    }
}

impl Display for Push<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // NOTE: does not work with vectors of mutable references.
        writeln!(
            f,
            "Vector.push_back<{}>(copy({}), {});",
            self.vector.kind().inner(),
            self.vector.identifier(),
            self.item
        )?;

        if let Some(increment) = &self.increment {
            increment.fmt(f)?;
        }

        Ok(())
    }
}
