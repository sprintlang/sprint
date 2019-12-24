use super::{context::Context, primitive::PRIMITIVES, unify::Unify, Result};
use crate::ast::{Definition, Expression, Kind, Variable};

pub fn program<'a>(definitions: Vec<Context<'a, Expression>>) -> Result<'a, Context<'a, ()>> {
    let mut context = Context::from(());

    context = definitions
        .into_iter()
        .fold(context, |mut context, definition| {
            context.unify(definition).expect("cannot unify contexts");
            context
        });

    context
        .unify(signature("main", Kind::State).unwrap())
        .expect("main function is not a contract");

    for variable in &context.variables {
        assert!(
            context.definitions.contains_key(variable.name),
            "no definition given for `{} :: {}`",
            variable.name,
            variable.kind
        );
    }

    Ok(context)
}

pub fn signature(identifier: &str, kind: Kind) -> Result<Context<Expression>> {
    let variable = Variable::new(identifier, kind.into());

    let mut context = Context::from(Expression::Variable(variable.clone()));
    context.variables.insert(variable);

    Ok(context)
}

pub fn definition<'a>(
    identifier: &'a str,
    arguments: Vec<&'a str>,
    mut expression: Context<'a, Expression<'a>>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    for argument in arguments.iter().rev() {
        let argument = Variable::new(argument, Default::default());
        let argument = expression.variables.take(&argument).unwrap_or(argument);
        expression = expression.map(|e| Expression::Abstraction(argument, e.into()));
    }

    let (expression, definition) = expression.clear();
    let variable = Variable::new(identifier, definition.kind());

    let mut context = Context::from(Expression::Variable(variable.clone()));

    let definition = Definition::new(variable.clone(), definition);
    context.definitions.insert(identifier, definition);

    context.variables.insert(variable);
    context.unify(expression).expect("cannot unify contexts");

    Ok(context)
}

pub fn application<'a>(
    identifier: &'a str,
    arguments: Vec<Context<'a, Expression<'a>>>,
) -> Result<'a, Context<'a, Expression<'a>>> {
    let (contexts, arguments): (Vec<_>, Vec<_>) = arguments.into_iter().map(Context::clear).unzip();

    let mut context = match PRIMITIVES.get(identifier) {
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
            let variable = Variable::new(identifier, kind.into());

            let mut context = Context::from(Expression::Variable(variable.clone()));
            context.variables.insert(variable);

            arguments.into_iter().fold(context, |context, argument| {
                context.map(|e| Expression::Application(e.into(), argument.into()))
            })
        }
    };

    for c in contexts {
        context.unify(c).expect("cannot unify contexts");
    }

    Ok(context)
}
