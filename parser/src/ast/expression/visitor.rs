use super::{
    class::{Comparable, Equatable, Numerable},
    kind::{Boolean, Observable, Word},
};

pub trait Visitor {
    fn visit_boolean(&mut self, value: &Boolean);

    fn visit_comparable(&mut self, value: &Comparable);

    fn visit_equatable(&mut self, value: &Equatable);

    fn visit_numerable(&mut self, value: &Numerable);

    fn visit_observable(&mut self, value: &Observable);

    fn visit_word(&mut self, value: &Word);
}

pub trait Accept {
    fn accept(&self, visitor: &mut dyn Visitor);
}
