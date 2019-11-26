use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/deposit.mvir", escape = "none")]
pub struct Deposit {
    pub author: String,
    pub module: String,
    pub owner: String,
}
