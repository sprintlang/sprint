use super::{
    action::{
        assert::Assert,
        assign::Assign,
        libra::{DestroyHandle, Emit},
        update_state::UpdateState,
        Action,
    },
    expression::Expression,
    identifier::Identifier,
    kind::Kind,
    variable::{
        Variable, CONTEXTS, CONTEXT_INDEX, CONTEXT_REF, CONTRACT_REF, EVENT, OWNER, STACK,
        STACK_LENGTH, TO_STATE,
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
    post_actions: Vec<Box<dyn Action + 'a>>,
    emit_actions: Vec<Box<dyn Action + 'a>>,
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
            post_actions: Default::default(),
            emit_actions: Default::default(),
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

    pub fn transition(name: &'a str, from: u64, to: Expression<'a>) -> Self {
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
            STACK.clone(),
            Expression::Expression("&mut copy(context_ref).stack".into()),
        ));

        method.add_action(Assign::new(
            STACK_LENGTH.clone(),
            Expression::Length(
                Kind::Unsigned,
                Expression::Identifier(STACK.identifier().clone())
                    .copy()
                    .freeze()
                    .into(),
            ),
        ));

        method.add_action(Assign::new(TO_STATE.clone(), to.stack_expansion()));

        method.add_argument(OWNER.clone());
        method.add_argument(CONTEXT_INDEX.clone());

        method.add_action(Assert::new(
            Expression::Expression(
                format!("*(&copy({}).state) == {}", CONTEXT_REF.identifier(), from).into(),
            ),
            1,
        ));

        method.add_post_action(UpdateState);

        method.add_emit_action(Assign::new(
            EVENT.clone(),
            Expression::Expression("LibraAccount.new_event_handle<u64>()".into()),
        ));
        method.add_emit_action(Emit::new(
            Expression::Identifier(TO_STATE.identifier().clone()).copy(),
        ));
        method.add_emit_action(DestroyHandle);

        method
    }

    pub fn dependencies(&self) -> Vec<&str> {
        self.all_actions()
            .into_iter()
            .flat_map(Action::dependencies)
            .copied()
            .collect()
    }

    pub fn definitions(&self) -> HashSet<&Variable> {
        self.all_actions()
            .into_iter()
            .flat_map(Action::definitions)
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

    pub fn add_post_action(&mut self, post_action: impl Action + 'a) {
        self.post_actions.push(Box::new(post_action));
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

    fn all_actions(&self) -> Vec<&(dyn Action + 'a)> {
        self.actions
            .iter()
            .chain(self.post_actions.iter())
            .chain(self.emit_actions.iter())
            .map(AsRef::as_ref)
            .collect()
    }

    fn add_emit_action(&mut self, emit_action: impl Action + 'a) {
        self.emit_actions.push(Box::new(emit_action));
    }
}
