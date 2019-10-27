mod class;

pub use self::class::{Class, Comparable, Equatable, Negatable, Numerable};

#[derive(PartialEq, Eq, Debug)]
pub enum Expression {
    Boolean(bool),
    Class(Class),
    Observable(Observable),
    Word(u64),
}

#[derive(PartialEq, Eq)]
pub enum Kind {
    Boolean,
    Observable(Box<Kind>),
    Word,
}

impl Expression {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Boolean(_) => Kind::Boolean,
            Self::Class(c) => match c {
                Class::Comparable(_) => Kind::Boolean,
                Class::Equatable(_) => Kind::Boolean,
                Class::Negatable(Negatable::Negate(e)) => e.kind(),
                Class::Numerable(n) => n.kind(),
            },
            Self::Observable(o) => Kind::Observable(Box::new(match o {
                Observable::IsHolder => Kind::Boolean,
                Observable::IsCounterparty => Kind::Boolean,
                Observable::Konst(e) => e.kind(),
            })),
            Self::Word(_) => Kind::Word,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Observable {
    IsHolder,
    IsCounterparty,
    Konst(Box<Expression>),
}

impl From<Expression> for Observable {
    fn from(e: Expression) -> Self {
        Self::Konst(Box::new(e))
    }
}
