use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Kind {
    Address,
    Coin,
    Context,
    Contract,
    EventHandle,
    MutableReference(Box<Self>),
    Unsigned,
    Vector(Box<Self>),
}

impl Kind {
    pub fn inner(&self) -> &Self {
        match self {
            Self::MutableReference(inner) => inner.inner(),
            Self::Vector(inner) => inner.inner(),
            _ => self,
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Address => write!(f, "address"),
            Self::Coin => write!(f, "LibraCoin.T"),
            Self::Context => write!(f, "Self.Context"),
            Self::Contract => write!(f, "Self.T"),
            Self::EventHandle => write!(f, "LibraAccount.EventHandle<u64>"),
            Self::MutableReference(kind) => write!(f, "&mut {}", kind),
            Self::Unsigned => write!(f, "u64"),
            Self::Vector(kind) => write!(f, "Vector.T<{}>", kind),
        }
    }
}
