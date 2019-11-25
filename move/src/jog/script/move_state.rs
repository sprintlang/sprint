use askama::Template;

#[derive(Template, Default)]
#[template(path = "scripts/go_to_state.mvir", escape = "none")]
pub struct MoveState {
    pub author: String,
    pub module: String,
    pub owner: String,
    pub context_id: u64,
    pub from_state: u64,
    pub to_state: u64,
}
