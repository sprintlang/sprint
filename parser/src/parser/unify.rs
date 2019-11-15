use super::context::Context;
use crate::ast::{Kind, Variable};
use std::rc::Rc;

pub trait Unify<O = Self> {
    fn unify(self, other: O) -> Option<()>;
}

impl<'a, T, U> Unify<&Context<'a, U>> for &mut Context<'a, T> {
    fn unify(self, other: &Context<'a, U>) -> Option<()> {
        self.definitions
            .extend(other.definitions.iter().map(|(&k, v)| (k, v.clone())));

        for (identifier, expression) in &other.variables {
            if let Some(original) = self.variables.insert(identifier, expression.clone()) {
                original.borrow_mut().unify(&mut *expression.borrow_mut())?;
            }
        }

        Some(())
    }
}

impl Unify for &mut Variable<'_> {
    fn unify(self, other: Self) -> Option<()> {
        match (&self, &other) {
            (Variable::Undefined(self_k), Variable::Undefined(other_k)) => {
                self_k.clone().unify(other_k.clone())?;
            }
            (Variable::Undefined(k), _) => {
                other.kind().unify(k.clone())?;
                *self = other.clone();
            }
            (_, Variable::Undefined(_)) => other.unify(self)?,
            _ => None?,
        };

        Some(())
    }
}

impl Unify for Rc<Kind> {
    fn unify(self, other: Self) -> Option<()> {
        let this = Kind::simplify(self);
        let other = Kind::simplify(other);

        match (this.as_ref(), other.as_ref()) {
            (Kind::Abstraction(this_from, this_to), Kind::Abstraction(other_from, other_to)) => {
                this_from.clone().unify(other_from.clone())?;
                this_to.clone().unify(other_to.clone())?;
            }
            (Kind::Boolean, Kind::Boolean) => {}
            (Kind::Observable(this_k), Kind::Observable(other_k)) => {
                this_k.clone().unify(other_k.clone())?;
            }
            (Kind::State, Kind::State) => {}
            (Kind::Unresolved(_), Kind::Unresolved(_)) if Rc::ptr_eq(&this, &other) => {}
            (Kind::Unresolved(k), _) if !Kind::contains(other.clone(), this.clone()) => {
                *k.borrow_mut() = Some(other);
            }
            (_, Kind::Unresolved(_)) => other.unify(this)?,
            (Kind::Word, Kind::Word) => {}
            _ => None?,
        }

        Some(())
    }
}
