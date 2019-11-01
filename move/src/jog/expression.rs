use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

#[derive(Default)]
pub struct Expression(Cow<'static, str>);

impl From<String> for Expression {
    fn from(expression: String) -> Self {
        Expression(expression.into())
    }
}

impl From<&'static str> for Expression {
    fn from(expression: &'static str) -> Self {
        Expression(expression.into())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}
