use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/transition.mvir", escape = "none")]
pub struct Transition<'a> {
    pub author: String,
    pub module: String,
    pub context_id: u64,
    pub function_names: &'a [&'a str],
}
