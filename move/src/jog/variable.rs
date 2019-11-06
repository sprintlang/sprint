use std::borrow::Cow;

#[derive(Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: Cow<'static, str>,
    pub type_name: &'static str,
    pub default: Option<Cow<'static, str>>,
}
