use super::{
    super::{
        visitor::{Accept, Visitor},
        Expression,
    },
    Boolean, Kind, Word,
};

#[derive(Debug)]
pub enum Observable {
    IsHolder,
    IsCounterparty,
    Konst(Box<dyn Expression>),
}

impl Expression for Observable {
    fn kind(&self) -> Kind {
        Kind::Observable(Box::new(match self {
            Self::IsHolder => Kind::Boolean,
            Self::IsCounterparty => Kind::Boolean,
            Self::Konst(value) => value.kind(),
        }))
    }
}

impl Accept for Observable {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_observable(self);
    }
}

macro_rules! from {
    ($value:ty) => {
        impl From<$value> for Observable {
            fn from(value: $value) -> Self {
                Self::Konst(Box::new(value))
            }
        }
    };
}

from!(Boolean);
from!(Word);
