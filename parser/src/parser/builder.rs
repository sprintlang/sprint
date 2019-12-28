use super::{
    context::Context,
    error::{CombinedError, SprintError},
    primitive::PRIMITIVES,
    unify::Unify,
    Result, Span,
};
use crate::ast::{Definition, Expression, ExpressionType, Kind, Variable};
use nom::{error::ErrorKind, Err};
use std::rc::Rc;

pub fn program<'a>(definitions: Vec<Context<'a, Expression<'a>>>) -> Result<'a, Context<'a, ()>> {
    let mut context = Context::from(());
    // println!("Definitions: {:#?}", definitions);
    context = definitions.into_iter().fold(Ok(context), unify_context)?;

    context
        .unify(signature(Span::new("main"), Kind::State).unwrap())
        .map_err(|err| Err::Error(err))?;

    for variable in &context.variables {
        if !context.definitions.contains_key(variable.name) {
            return Err(Err::Error(CombinedError::from_sprint_error_and_error_kind(
                variable.span,
                ErrorKind::Tag,
                SprintError::UnknownIdentifierError(
                    variable.name,
                    Rc::make_mut(&mut variable.kind.clone()).clone(),
                ),
            )));
        }
    }

    Ok(context)
}

pub fn unify_context<'a>(
    context: Result<'a, Context<'a, ()>>,
    definition: Context<'a, Expression<'a>>,
) -> Result<'a, Context<'a, ()>> {
    let span = definition.as_ref().span;
    // println!("Definition in unify context: {:#?}", definition);
    match context {
        Err(_) => context,
        Ok(mut c) => match c.unify(definition) {
            Ok(_) => Ok(c),
            Err(e) => Err(Err::Error(e)),
        }
        // Ok(mut c) => match c.unify(definition) {
        //     Ok(_) => Ok(c),
        //     Err(e) => {
        //         println!("Span in error: {:#?}", span);
        //         return Err(Err::Error(CombinedError::from_sprint_error_and_error_kind(
        //             span,
        //             ErrorKind::Tag,
        //             e,
        //         )));
        //     }
        // },
    }
}

pub fn signature(identifier: Span, kind: Kind) -> Result<Context<Expression>> {
    let variable = Variable::new(identifier.fragment, kind.into(), identifier);

    let mut context = Context::from(Expression::new(
        ExpressionType::Variable(variable.clone()),
        identifier,
    ));
    context.variables.insert(variable);

    Ok(context)
}

pub fn definition<'a>(
    identifier: Span<'a>,
    arguments: Vec<Span<'a>>,
    mut expression: Context<'a, Expression<'a>>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    for argument in arguments.iter().rev() {
        let argument = Variable::new(argument.fragment, Default::default(), *argument);
        let argument = expression.variables.take(&argument).unwrap_or(argument);

        expression = expression.map(|e| {
            let span = e.span;
            // println!("span: {:#?}", span);
            Expression::new(ExpressionType::Abstraction(argument, e.into()), span)
        });
        // println!("Expression: {:#?}", expression);
    }

    let (expression, definition) = expression.clear();
    let variable = Variable::new(
        identifier.fragment,
        definition.expression.kind(),
        identifier,
    );

    let mut context = Context::from(Expression::new(
        ExpressionType::Variable(variable.clone()),
        identifier,
    ));

    let definition = Definition::new(variable.clone(), definition);

    context.definitions.insert(identifier.fragment, definition);
    context.variables.insert(variable);

    // TODO: Bespoke sprint error?
    // context.unify(expression).map_err(|sprint_error| {
    //     Err::Error(CombinedError::from_sprint_error_and_error_kind(
    //         identifier,
    //         ErrorKind::Tag,
    //         sprint_error,
    //     ))
    // })?;
    context.unify(expression).map_err(|err| Err::Error(err))?;

    Ok(context)
}

pub fn application<'a>(
    identifier: Span<'a>,
    arguments: Vec<Context<'a, Expression<'a>>>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    let (contexts, arguments): (Vec<_>, Vec<_>) = arguments.into_iter().map(Context::clear).unzip();
    let mut context = match PRIMITIVES.get(identifier.fragment) {
        // Applications to primitives are handled here. Primitives cannot be
        // implemented as abstractions in the AST as some require polymorphic
        // kinds which is not currently supported.
        Some(primitive) => primitive(arguments)?,
        _ => {
            let kind = arguments
                .iter()
                .rev()
                .fold(Kind::default(), |kind, argument| {
                    Kind::Abstraction(argument.expression.kind(), kind.into())
                });
            let variable = Variable::new(identifier.fragment, kind.into(), identifier);

            let mut context = Context::from(Expression::new(
                ExpressionType::Variable(variable.clone()),
                identifier,
            ));
            context.variables.insert(variable);
            arguments.into_iter().fold(Ok(context), map_args)?
        }
    };

    for c in contexts {
        // TODO: bespoke sprint error?
        // context.unify(c).map_err(|sprint_error| {
        //     Err::Error(CombinedError::from_sprint_error_and_error_kind(
        //         identifier,
        //         ErrorKind::Tag,
        //         sprint_error,
        //     ))
        // })?;
        context.unify(c).map_err(|err| Err::Error(err))?;
    }

    Ok(context)
}

pub fn map_args<'a>(
    context: Result<'a, Context<'a, Expression<'a>>>,
    argument: Expression<'a>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    match context {
        Err(_) => context,
        Ok(c) => {
            let tmp = c.map(|e| {
                let span = e.span;
                Expression::new(ExpressionType::Application(e.into(), argument.into()), span)
            });
            // println!("Context: {:#?}", tmp);
            Ok(tmp)
        }
    }
}
