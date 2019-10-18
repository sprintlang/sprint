use super::{
    super::{
        visitor::{Accept, Visitor},
        Expression,
    },
    Kind,
};

#[derive(Default, Debug)]
pub struct Boolean(pub bool);

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Expression for Boolean {
    fn kind(&self) -> Kind {
        Kind::Boolean
    }
}

impl Accept for Boolean {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_boolean(self);
    }
}
