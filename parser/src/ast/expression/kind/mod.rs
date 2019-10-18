mod boolean;
mod observable;
mod word;

pub use self::{boolean::Boolean, observable::Observable, word::Word};

#[derive(PartialEq, Eq)]
pub enum Kind {
    Boolean,
    Observable(Box<Kind>),
    Word,
}
