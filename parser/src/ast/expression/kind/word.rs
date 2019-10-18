use super::{
    super::{
        visitor::{Accept, Visitor},
        Expression,
    },
    Kind,
};

#[derive(Default, Debug)]
pub struct Word(pub u64);

impl From<u64> for Word {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Expression for Word {
    fn kind(&self) -> Kind {
        Kind::Word
    }
}

impl Accept for Word {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_word(self);
    }
}
