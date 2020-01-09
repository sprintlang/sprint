use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/go_to_state.mvir", escape = "none")]
pub struct MoveState<'a> {
    pub author: String,
    pub module: String,
    pub context_id: u64,
    pub to_states: &'a [&'a str],
}
