use std::fmt::{self, Display, Formatter};

const PREFIX: &str = "s_";
const SPAWN: &str = "spawn_";

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Identifier<'a> {
    Raw(&'a str),
    Prefixed(&'a str),
    Spawn(usize),
    Transition(usize, usize),
}

impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Raw(name) => name.fmt(f),
            Self::Prefixed(name) => write!(f, "{}{}", PREFIX, name),
            Self::Spawn(num) => write!(f, "{}{}", SPAWN, num),
            Self::Transition(from, to) => write!(f, "transition_{}_{}", from, to),
        }
    }
}
