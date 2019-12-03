use super::context::Context;
use crate::ast::Kind;
use std::rc::Rc;

pub trait Unify<O = Self> {
    fn unify(self, other: O) -> Option<()>;
}

impl<'a, T, U> Unify<Context<'a, U>> for &mut Context<'a, T> {
    fn unify(self, other: Context<'a, U>) -> Option<()> {
        for (name, definition) in other.definitions {
            if self.definitions.insert(name, definition).is_some() {
                // There is a duplicate definition.
                return None;
            }
        }

        for variable in other.variables {
            let kind = variable.kind.clone();

            if let Some(original) = self.variables.replace(variable) {
                original.kind.unify(kind);
            }
        }

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
