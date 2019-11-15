use std::fmt::{self, Display, Formatter};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Kind {
    Address,
    Coin,
    Context,
    Contexts,
    Unsigned,
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Address => write!(f, "address"),
            Self::Coin => write!(f, "LibraCoin.T"),
            Self::Context => write!(f, "&mut Self.Context"),
            Self::Contexts => write!(f, "&mut Vector.T<Self.Context>"),
            Self::Unsigned => write!(f, "u64"),
        }
    }
}
