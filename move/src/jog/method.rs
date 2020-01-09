use super::{
    action::{assert::Assert, assign::Assign, Action},
    expression::Expression,
    identifier::Identifier,
    kind::Kind,
    variable::{
        Variable, CONTEXTS, CONTEXT_INDEX, CONTEXT_REF, CONTRACT_REF, OWNER, STACK, STACK_LENGTH,
    },
};
use askama::Template;
use std::collections::HashSet;

#[derive(Template, Debug)]
#[template(path = "method.mvir", escape = "none")]
pub struct Method<'a> {
    public: bool,
    identifier: Identifier<'a>,
    arguments: Vec<Variable<'a>>,
    actions: Vec<Box<dyn Action + 'a>>,
    result: Option<Expression<'a>>,
    acquires_resource: bool,
}

impl<'a> Method<'a> {
    fn new(public: bool, identifier: Identifier<'a>) -> Self {
        Self {
            public,
            identifier,
            arguments: Default::default(),
            actions: Default::default(),
            result: Default::default(),
            acquires_resource: false,
        }
    }

    pub fn private(identifier: Identifier<'a>) -> Self {
        Self::new(false, identifier)
    }

    pub fn public(identifier: Identifier<'a>) -> Self {
        Self::new(true, identifier)
    }

    pub fn transition(name: &'a str, arguments: &[Variable<'a>], from: usize) -> Self {
        let mut method = Self::public(Identifier::Transition(name));

        method.add_action(Assign::new(
            CONTRACT_REF.clone(),
            Expression::Expression(
                format!("borrow_global_mut<T>(move({}))", OWNER.identifier()).into(),
            ),
        ));
        method.set_acquires_resource();

        method.add_action(Assign::new(
            CONTEXTS.clone(),
            Expression::Expression("&mut copy(contract_ref).contexts".into()),
        ));

        method.add_action(Assign::new(
            CONTEXT_REF.clone(),
            Expression::Expression(
                format!(
                    "Vector.borrow_mut<Self.Context>(copy({}), copy({}))",
                    CONTEXTS.identifier(),
                    CONTEXT_INDEX.identifier()
                )
                .into(),
            ),
        ));

        method.add_action(Assign::new(
            STACK_LENGTH.clone(),
            Expression::Expression("Vector.length<u64>(&copy(context_ref).stack)".into()),
        ));

        for (i, argument) in arguments.iter().rev().enumerate() {
            method.add_action(Assign::new(
                argument.clone(),
                Expression::Get(
                    Kind::Unsigned,
                    Expression::Expression("&copy(context_ref).stack".into()).into(),
                    Expression::Subtract(
                        Expression::Copied(
                            Expression::Identifier(STACK_LENGTH.identifier().clone()).into(),
                        )
                        .into(),
                        Expression::Unsigned(i + 1).into(),
                    )
                    .into(),
                ),
            ));
        }

        method.add_action(Assign::new(
            STACK.clone(),
            Expression::Expression("&mut copy(context_ref).stack".into()),
        ));

        method.add_argument(OWNER.clone());
        method.add_argument(CONTEXT_INDEX.clone());

        method.add_action(Assert::new(
            Expression::Expression(
                format!("*(&copy({}).state) == {}", CONTEXT_REF.identifier(), from).into(),
            ),
            1,
        ));

        method
    }

    pub fn dependencies(&self) -> Vec<&str> {
        self.actions
            .iter()
            .flat_map(|action| action.dependencies())
            .copied()
            .collect()
    }

    pub fn definitions(&self) -> HashSet<&Variable> {
        self.actions
            .iter()
            .flat_map(|action| action.definitions())
            .collect()
    }

    pub fn add_argument(&mut self, argument: Variable<'a>) {
        self.arguments.push(argument);
    }

    pub fn set_arguments(&mut self, arguments: Vec<Variable<'a>>) {
        assert!(self.arguments.is_empty());
        self.arguments = arguments;
    }

    pub fn add_action(&mut self, action: impl Action + 'a) {
        self.actions.push(Box::new(action));
    }

    pub fn set_result(&mut self, expression: Expression<'a>) {
        self.result = Some(expression);
    }

    pub fn set_acquires_resource(&mut self) {
        self.acquires_resource = true;
    }

    fn result(&self) -> String {
        self.result
            .as_ref()
            .map(|e| format!(" {}", e))
            .unwrap_or_default()
    }
}
