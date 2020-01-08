use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/create.mvir", escape = "none")]
pub struct CreateContract {
    pub author: String,
    pub module: String,
}
