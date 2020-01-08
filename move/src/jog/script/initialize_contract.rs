use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/initialize.mvir", escape = "none")]
pub struct InitializeContract {
    pub author: String,
    pub module: String,
    pub party: String,
    pub counterparty: String,
}
