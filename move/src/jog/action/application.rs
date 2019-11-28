use super::{
    super::{application, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Application<'a>(application::Application<'a>);

impl<'a> From<application::Application<'a>> for Application<'a> {
    fn from(application: application::Application<'a>) -> Self {
        Self(application)
    }
}

impl Action for Application<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &[]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for Application<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)?;
        write!(f, ";")
    }
}
