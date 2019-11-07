pub mod action;
pub mod expression;
pub mod method;
pub mod module;
pub mod variable;

pub fn generate_context_name(context_id: usize) -> String {
    format!("context_{}", context_id)
}
