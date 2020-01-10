use std::fmt::{self, Display, Formatter};

const PREFIX: &str = "s";
const SPAWN: &str = "spawn";

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum Identifier<'a> {
    Raw(&'a str),
    Prefixed(&'a str),
    Spawn(u64),
    Transition(&'a str),
}

impl Identifier<'_> {
    pub fn has_name(&self, needle: &str) -> bool {
        match &self {
            Self::Raw(name) | Self::Prefixed(name) | Self::Transition(name) => *name == needle,
            _ => false,
        }
    }
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Raw(name) => name.fmt(f),
            Self::Prefixed(name) => write!(f, "{}_{}", PREFIX, name),
            Self::Spawn(id) => write!(f, "{}_{}", SPAWN, id),
            Self::Transition(name) => write!(f, "transition_{}", name),
        }
    }
}
