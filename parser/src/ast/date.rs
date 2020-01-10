#[derive(Clone, Debug)]
pub enum Date {
    Now,
    Date(u64, u64, u64, u64, u64, u64),
}
