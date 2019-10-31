#[derive(Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub type_name: &'static str,
    pub default: Option<String>,
}
