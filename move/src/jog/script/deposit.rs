use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/deposit.mvir", escape = "none")]
pub struct Deposit {
    pub author: String,
    pub module: String,
    pub amount: String,
    pub coin_store_index: u64,
}
