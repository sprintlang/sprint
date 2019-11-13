use super::module::Contract;
use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter},
};

// pub type Expression = Cow<'static, str>;

pub enum Expression<'a> {
    Expression(Cow<'static, str>),
    Contract(Contract<'a>),
}

impl Default for Expression<'_> {
    fn default() -> Self {
        Self::Expression(Cow::default())
    }
}

impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "{}", e),
            Self::Contract(c) => write!(f, "{}", c),
        }
    }
}
