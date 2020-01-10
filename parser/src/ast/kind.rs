use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display},
    rc::Rc,
};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Kind {
    Abstraction(Rc<Self>, Rc<Self>),
    Boolean,
    Date,
    Observable(Rc<Self>),
    State,
    Unresolved(RefCell<Option<Rc<Self>>>),
    Word,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Unresolved(None.into())
    }
}

impl Kind {
    pub fn simplify(mut kind: Rc<Kind>) -> Rc<Kind> {
        while let Kind::Unresolved(k) = kind.clone().as_ref() {
            match k.borrow().as_ref() {
                Some(k) => kind = k.clone(),
                None => break,
            }
        }

        kind
    }

    pub fn contains(this: Rc<Kind>, other: Rc<Kind>) -> bool {
        if Rc::ptr_eq(&this, &other) {
            return true;
        }

        match this.as_ref() {
            Self::Abstraction(from, to) => {
                Self::contains(from.clone(), other.clone()) || Self::contains(to.clone(), other)
            }
            Self::Boolean => false,
            Self::Date => false,
            Self::Observable(k) => Self::contains(k.clone(), other),
            Self::State => false,
            Self::Unresolved(k) => match k.borrow().as_ref() {
                Some(k) => Self::contains(k.clone(), other),
                None => false,
            },
            Self::Word => false,
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Formatter::new(self).fmt(f)
    }
}

struct Formatter<'a> {
    kind: &'a Kind,
    symbols: Rc<RefCell<HashMap<*const Kind, char>>>,
}

impl<'a> Formatter<'a> {
    fn new(kind: &'a Kind) -> Self {
        Formatter {
            kind,
            symbols: Default::default(),
        }
    }

    fn with(&self, kind: &'a Kind) -> Self {
        Formatter {
            kind,
            symbols: self.symbols.clone(),
        }
    }

    fn symbol(&self) -> char {
        let key = self.kind as *const _;
        let mut symbols = self.symbols.borrow_mut();

        match symbols.get(&key) {
            Some(&symbol) => symbol,
            None => {
                const FIRST: u8 = b'a';

                let symbol = (FIRST + symbols.len() as u8) as char;
                symbols.insert(key, symbol);

                symbol
            }
        }
    }
}

impl Display for Formatter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            Kind::Abstraction(from, to) => {
                let from = Kind::simplify(from.clone());

                match from.as_ref() {
                    Kind::Abstraction(_, _) => write!(f, "({})", self.with(&from)),
                    _ => write!(f, "{}", self.with(&from)),
                }?;

                write!(f, " -> {}", self.with(to))
            }
            Kind::Boolean => write!(f, "Bool"),
            Kind::Date => write!(f, "Date"),
            Kind::Observable(k) => write!(f, "Observable {}", self.with(k)),
            Kind::State => write!(f, "Contract"),
            Kind::Unresolved(k) => match k.borrow().as_ref() {
                Some(k) => self.with(k).fmt(f),
                None => write!(f, "{}", self.symbol()),
            },
            Kind::Word => write!(f, "Word"),
        }
    }
}
