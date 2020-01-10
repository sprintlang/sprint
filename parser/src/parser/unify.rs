use super::{
    context::Context,
    error::{Error, SprintError},
};
use crate::ast::Kind;
use std::rc::Rc;

pub trait Unify<'a, O = Self> {
    fn unify(self, other: O) -> Result<(), Error<'a>>;
}

impl<'a, T, U> Unify<'a, Context<'a, U>> for &mut Context<'a, T> {
    fn unify(self, other: Context<'a, U>) -> Result<(), Error<'a>> {
        for (name, definition) in &other.definitions {
            if self.definitions.insert(name, definition.clone()).is_some() {
                // There is a duplicate definition.
                return Err(Error::from_sprint_error(
                    SprintError::DuplicateDefinitionError(name),
                    definition.expression.span,
                ));
            }
        }

        for (variable, count) in other.variables {
            if let Some(original) = self.variables.replace(variable.clone(), count) {
                if let Err(e) = original.kind.unify(variable.kind.clone()) {
                    let sprint_error =
                        SprintError::TypeError(variable.name, e.sprint_error.unwrap().into());
                    return Err(Error::from_sprint_error(sprint_error, variable.span));
                }
            }
        }

        Ok(())
    }
}

impl<'a> Unify<'a> for Rc<Kind> {
    fn unify(self, other: Self) -> Result<(), Error<'a>> {
        let mut this = Kind::simplify(self);
        let mut other = Kind::simplify(other);

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
            _ => {
                return Err(Error::from_sprint_error(
                    SprintError::MismatchedKinds(
                        Rc::make_mut(&mut this).clone(),
                        Rc::make_mut(&mut other).clone(),
                    ),
                    None,
                ))
            }
        }

        Ok(())
    }
}
