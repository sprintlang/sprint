use super::{
    context::Context,
    error::SprintError,
    primitive::{self, PRIMITIVES},
    unify::Unify,
    Error, Result, Span,
};
use crate::ast::{Definition, Expression, ExpressionType, Kind, Variable};
use nom::Err;
use std::rc::Rc;

pub fn program<'a>(definitions: Vec<Context<'a, Expression<'a>>>) -> Result<'a, Context<'a, ()>> {
    let mut context = Context::from(());

    context.unify(primitive::zero()).unwrap();
    context.unify(primitive::one()).unwrap();
    context.unify(primitive::give()).unwrap();
    context.unify(primitive::and()).unwrap();
    context.unify(primitive::or()).unwrap();
    context.unify(primitive::before()).unwrap();
    context.unify(primitive::after()).unwrap();
    context.unify(primitive::scale()).unwrap();
    context.unify(primitive::anytime()).unwrap();

    context = definitions.into_iter().fold(Ok(context), unify_context)?;
    context
        .unify(signature(Span::new("main"), Kind::State).unwrap())
        .map_err(Err::Error)?;

    for (variable, _) in &context.variables {
        if !context.definitions.contains_key(variable.name) {
            if variable.name == "main" {
                return Err(Err::Error(Error::from_sprint_error(
                    SprintError::UndefinedMainError,
                    None,
                )));
            }
            return Err(Err::Error(Error::from_sprint_error(
                SprintError::UnknownIdentifierError(
                    variable.name,
                    Rc::make_mut(&mut variable.kind.clone()).clone(),
                ),
                variable.span,
            )));
        }
    }

    Ok(context)
}

pub fn signature(identifier: Span, kind: Kind) -> Result<Context<Expression>> {
    let variable = Variable::new(identifier.fragment, kind.into(), Some(identifier));

    let mut context = Context::from(Expression::new(
        ExpressionType::Variable(variable.clone()),
        Some(identifier),
    ));
    context.variables.insert_without_increment(variable);

    Ok(context)
}

pub fn definition<'a>(
    identifier: Span<'a>,
    arguments: Vec<Span<'a>>,
    mut expression: Context<'a, Expression<'a>>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    for argument in arguments.iter().rev() {
        let argument = Variable::new(argument.fragment, Default::default(), Some(*argument));
        let argument = expression.variables.take(&argument).unwrap_or(argument);

        expression = expression.map(|expression| {
            Expression::new(
                ExpressionType::Abstraction(argument, expression.clone().into()),
                expression.span,
            )
        });
    }

    let (expression, definition) = expression.clear();
    let variable = Variable::new(identifier.fragment, definition.kind(), Some(identifier));

    let mut context = Context::from(Expression::new(
        ExpressionType::Variable(variable.clone()),
        Some(identifier),
    ));

    let definition = Definition::new(variable.clone(), definition);

    context.definitions.insert(identifier.fragment, definition);
    context.variables.insert(variable);

    context.unify(expression).map_err(Err::Error)?;

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
        Some(primitive) => primitive(arguments),
        _ => {
            let kind = arguments
                .iter()
                .rev()
                .fold(Kind::default(), |kind, argument| {
                    Kind::Abstraction(argument.kind(), kind.into())
                });
            let variable = Variable::new(identifier.fragment, kind.into(), Some(identifier));

            let mut context = Context::from(Expression::new(
                ExpressionType::Variable(variable.clone()),
                Some(identifier),
            ));
            context.variables.insert(variable);
            arguments
                .into_iter()
                .fold(Ok(context), map_arg_to_application)?
        }
    };

    for c in contexts {
        context.unify(c).map_err(Err::Error)?;
    }

    Ok(context)
}

fn unify_context<'a>(
    context: Result<'a, Context<'a, ()>>,
    definition: Context<'a, Expression<'a>>,
) -> Result<'a, Context<'a, ()>> {
    match context {
        Err(_) => context,
        Ok(mut context) => match context.unify(definition) {
            Ok(_) => Ok(context),
            Err(error) => Err(Err::Error(error)),
        },
    }
}

fn map_arg_to_application<'a>(
    context: Result<'a, Context<'a, Expression<'a>>>,
    argument: Expression<'a>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    match context {
        Err(_) => context,
        Ok(context) => {
            let context = context.map(|expression| {
                Expression::new(
                    ExpressionType::Application(expression.clone().into(), argument.into()),
                    expression.span,
                )
            });
            Ok(context)
        }
    }
}
