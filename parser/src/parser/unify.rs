use super::{
    context::Context,
    error::{CombinedError, SprintError},
};
use crate::ast::Kind;
use std::rc::Rc;

pub trait Unify<'a, O = Self> {
    fn unify(self, other: O) -> Result<(), CombinedError<'a>>;
}

impl<'a, T, U> Unify<'a, Context<'a, U>> for &mut Context<'a, T> {
    fn unify(self, other: Context<'a, U>) -> Result<(), CombinedError<'a>> {
        for (name, definition) in &other.definitions {
            if self.definitions.insert(name, definition.clone()).is_some() {
                // There is a duplicate definition.
                // TODO: duplicate definition error.
                return Err(CombinedError::from_sprint_error(
                    SprintError::InvalidNumberArgsError,
                ));
            }
        }

        for variable in other.variables {
            let name = variable.name;
            let kind = variable.kind.clone();

            if let Some(original) = self.variables.replace(variable) {
                if let Err(e) = original.kind.unify(kind) {
                    let def = self.definitions.get(name).unwrap();
                    let span = def.expression.span;
                    return Err(CombinedError::from_sprint_error_and_span(
                        span,
                        SprintError::TypeError(name, e.sprint_error.unwrap().into()),
                    ));
                }
            }
        }

        Ok(())
    }
}

impl<'a> Unify<'a> for Rc<Kind> {
    fn unify(self, other: Self) -> Result<(), CombinedError<'a>> {
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
                return Err(CombinedError::from_sprint_error(
                    SprintError::MismatchedKinds(
                        Rc::make_mut(&mut this).clone(),
                        Rc::make_mut(&mut other).clone(),
                    ),
                ))
            }
        }

        Ok(())
    }
}
