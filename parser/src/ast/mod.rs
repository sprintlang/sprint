pub mod state;

mod class;

pub use self::class::{Class, Comparable, Equatable, Negatable, Numerable};

#[derive(PartialEq, Eq, Debug)]
pub enum Expression<'a> {
    Application(Box<Expression<'a>>, Box<Expression<'a>>),
    Boolean(bool),
    Class(Class<'a>),
    Identifier(&'a str, Kind),
    Observable(Observable<'a>),
    Word(u64),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Kind {
    Abstraction(Box<Kind>, Box<Kind>),
    Boolean,
    Observable(Box<Kind>),
    Word,
}

impl Expression<'_> {
    pub fn kind(&self) -> Kind {
        match self {
            Self::Application(f, _) => match f.kind() {
                Kind::Abstraction(_, k) => k.as_ref().clone(),
                _ => unreachable!(),
            },

            Self::Boolean(_) => Kind::Boolean,

            Self::Class(c) => match c {
                Class::Comparable(_) => Kind::Boolean,
                Class::Equatable(_) => Kind::Boolean,
                Class::Negatable(Negatable::Negate(e)) => e.kind(),
                Class::Numerable(n) => n.kind(),
            },

            Self::Identifier(_, k) => k.clone(),

            Self::Observable(o) => Kind::Observable(Box::new(match o {
                Observable::IsHolder => Kind::Boolean,
                Observable::IsCounterparty => Kind::Boolean,
                Observable::Konst(e) => e.kind(),
            })),

            Self::Word(_) => Kind::Word,
        }
    }
}

impl From<bool> for Expression<'_> {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl<'a> From<Class<'a>> for Expression<'a> {
    fn from(c: Class<'a>) -> Self {
        Self::Class(c)
    }
}

impl<'a> From<Observable<'a>> for Expression<'a> {
    fn from(o: Observable<'a>) -> Self {
        Self::Observable(o)
    }
}

impl From<u64> for Expression<'_> {
    fn from(w: u64) -> Self {
        Self::Word(w)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum Observable<'a> {
    IsHolder,
    IsCounterparty,
    Konst(Box<Expression<'a>>),
}

impl<'a> From<Expression<'a>> for Observable<'a> {
    fn from(e: Expression<'a>) -> Self {
        Self::Konst(e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kind_of_application() {
        // one :: Word -> Word -> Boolean
        let one = Expression::Identifier(
            "one",
            Kind::Abstraction(
                Kind::Word.into(),
                Kind::Abstraction(Kind::Word.into(), Kind::Boolean.into()).into(),
            ),
        );

        // two = one 42
        let two = Expression::Application(one.into(), Expression::Word(42).into());

        // two :: Word -> Boolean
        assert_eq!(
            two.kind(),
            Kind::Abstraction(Kind::Word.into(), Kind::Boolean.into())
        );

        // three = two 29
        let three = Expression::Application(two.into(), Expression::Word(29).into());

        // three :: Boolean
        assert_eq!(three.kind(), Kind::Boolean);
    }
}
