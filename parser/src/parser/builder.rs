use super::{context::Context, primitive::PRIMITIVES, unify::Unify, Result};
use crate::ast::{Argument, Definition, Expression, Kind, Variable};
use std::{cell::RefCell, rc::Rc};

pub fn program<'a>(definitions: Vec<Context<'a, Expression>>) -> Result<Context<'a, ()>> {
    let mut context = Context::from(());

    context = definitions
        .into_iter()
        .fold(context, |mut context, definition| {
            context.unify(&definition).expect("cannot unify contexts");
            context
        });

    context
        .unify(&signature("main", Kind::State).unwrap())
        .expect("main function is not a contract");

    for (identifier, variable) in &context.variables {
        assert!(
            context.definitions.contains_key(identifier),
            "no definition given for `{} :: {}`",
            identifier,
            variable.borrow().kind()
        );
    }

    Ok(context)
}

pub fn signature(identifier: &str, kind: Kind) -> Result<Context<Expression>> {
    let variable = Rc::new(RefCell::new(Variable::Undefined(kind.into())));

    let mut context = Context::from(Expression::Variable(variable.clone()));
    context.variables.insert(identifier, variable);

    Ok(context)
}

pub fn definition<'a>(
    identifier: &'a str,
    arguments: Vec<&'a str>,
    mut expression: Context<'a, Expression>,
) -> Result<'a, Context<'a, Expression>> {
    for argument in arguments.iter().rev() {
        let variable = expression.variables.remove(argument);

        let argument = match variable {
            Some(variable) => {
                let argument = Rc::new(Argument(variable.borrow().kind()));
                *variable.borrow_mut() = Variable::Argument(argument.clone());
                argument
            }
            None => Rc::new(Argument(Kind::default().into())),
        };

        expression = expression.map(|e| Expression::Abstraction(argument, e.into()));
    }

    let (expression, definition) = expression.clear();
    let definition = Rc::new(Definition::from(definition));

    let reference = Rc::new(RefCell::new(Variable::from(&definition)));

    let mut context = Context::from(Expression::Variable(reference.clone()));
    context.definitions.insert(identifier, definition);
    context.variables.insert(identifier, reference);

    context.unify(&expression).expect("cannot unify contexts");

    Ok(context)
}

pub fn application<'a>(
    identifier: &'a str,
    arguments: Vec<Context<'a, Expression>>,
) -> Result<'a, Context<'a, Expression>> {
    let (contexts, arguments): (Vec<_>, Vec<_>) = arguments.into_iter().map(Context::clear).unzip();

    let mut context = match PRIMITIVES.get(identifier) {
        // Applications to primitives are handled here. Primitives cannot be
        // implemented as abstractions in the AST as some require polymorphic
        // kinds which is not currently supported.
        Some(primitive) => primitive.build(arguments),
        _ => {
            let kind = arguments
                .iter()
                .rev()
                .fold(Kind::default(), |kind, argument| {
                    Kind::Abstraction(argument.kind(), kind.into())
                });

            let reference = Rc::new(RefCell::new(Variable::Undefined(kind.into())));

            let mut context = Context::from(Expression::Variable(reference.clone()));
            context.variables.insert(identifier, reference);

            arguments.into_iter().fold(context, |context, argument| {
                context.map(|e| Expression::Application(e.into(), argument.into()))
            })
        }
    };

    for c in contexts {
        context.unify(&c).expect("cannot unify contexts");
    }

    Ok(context)
}
