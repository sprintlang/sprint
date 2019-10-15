#[derive(Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: &'static str,
    pub type_name: &'static str,
    pub default: Option<&'static str>,
}
