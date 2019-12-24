use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Kind {
    Address,
    Coin,
    Context,
    Contexts,
    Contract,
    MutableReference(Box<Self>),
    Unsigned,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Address => write!(f, "address"),
            Self::Coin => write!(f, "LibraCoin.T"),
            Self::Context => write!(f, "Self.Context"),
            Self::Contexts => write!(f, "Vector.T<Self.Context>"),
            Self::Contract => write!(f, "Self.T"),
            Self::MutableReference(kind) => write!(f, "&mut {}", kind),
            Self::Unsigned => write!(f, "u64"),
        }
    }
}
