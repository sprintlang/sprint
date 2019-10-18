use super::{
    kind::Kind,
    visitor::{Accept, Visitor},
    Expression,
};

pub trait Class: Expression {
    fn instance(kind: &Kind) -> bool;
}

#[derive(Debug)]
pub enum Comparable {
    Greater(Box<dyn Expression>, Box<dyn Expression>),
    Less(Box<dyn Expression>, Box<dyn Expression>),
    GreaterEqual(Box<dyn Expression>, Box<dyn Expression>),
    LessEqual(Box<dyn Expression>, Box<dyn Expression>),
}

impl Class for Comparable {
    fn instance(kind: &Kind) -> bool {
        Numerable::instance(kind)
    }
}

impl Expression for Comparable {
    fn kind(&self) -> Kind {
        Kind::Boolean
    }
}

impl Accept for Comparable {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_comparable(self);
    }
}

#[derive(Debug)]
pub enum Equatable {
    Equal(Box<dyn Expression>, Box<dyn Expression>),
    NotEqual(Box<dyn Expression>, Box<dyn Expression>),
}

impl Class for Equatable {
    fn instance(kind: &Kind) -> bool {
        Comparable::instance(kind)
            || match kind {
                Kind::Boolean => true,
                _ => false,
            }
    }
}

impl Expression for Equatable {
    fn kind(&self) -> Kind {
        Kind::Boolean
    }
}

impl Accept for Equatable {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_equatable(self);
    }
}

#[derive(Debug)]
pub enum Numerable {
    Add(Box<dyn Expression>, Box<dyn Expression>),
    Subtract(Box<dyn Expression>, Box<dyn Expression>),
    Multiply(Box<dyn Expression>, Box<dyn Expression>),
    Divide(Box<dyn Expression>, Box<dyn Expression>),
}

impl Class for Numerable {
    fn instance(kind: &Kind) -> bool {
        match kind {
            Kind::Word => true,
            _ => false,
        }
    }
}

impl Expression for Numerable {
    fn kind(&self) -> Kind {
        match self {
            Self::Add(n, _) | Self::Subtract(n, _) | Self::Multiply(n, _) | Self::Divide(n, _) => {
                n.kind()
            }
        }
    }
}

impl Accept for Numerable {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_numerable(self);
    }
}
