use super::{
    super::{call, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Call<'a>(call::Call<'a>);

impl<'a> From<call::Call<'a>> for Call<'a> {
    fn from(call: call::Call<'a>) -> Self {
        Self(call)
    }
}

impl Action for Call<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for Call<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)?;
        write!(f, ";")
    }
}
