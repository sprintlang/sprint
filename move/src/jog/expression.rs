use super::{abstraction::Abstraction, module::Contract};
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

pub enum Expression<'a> {
    Abstraction(Abstraction<'a>),
    Contract(Contract<'a>),
    Expression(Cow<'static, str>),
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Self::Expression(Cow::default())
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Abstraction(_) => unimplemented!(),
            Self::Contract(c) => write!(f, "{}", c),
            Self::Expression(e) => write!(f, "{}", e),
        }
    }
}
