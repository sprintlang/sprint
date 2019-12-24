use super::Expression;

#[derive(Default, Debug)]
pub struct State<'a> {
    transitions: Vec<Transition<'a>>,
}

impl<'a> State<'a> {
    pub fn transitions(&self) -> &[Transition<'a>] {
        &self.transitions
    }

    pub fn add_transition(&mut self, transition: Transition<'a>) -> &mut Self {
        self.transitions.push(transition);
        self
    }
}

#[derive(Default, Debug)]
pub struct Transition<'a> {
    conditions: Vec<Expression<'a>>,
    effects: Vec<Effect<'a>>,
    next: Option<Expression<'a>>,
}

impl<'a> Transition<'a> {
    pub fn conditions(&self) -> &[Expression<'a>] {
        &self.conditions
    }

    pub fn add_condition(&mut self, condition: Expression<'a>) -> &mut Self {
        self.conditions.push(condition);
        self
    }

    pub fn effects(&self) -> Vec<&Effect<'a>> {
        self.effects.iter().rev().collect()
    }

    pub fn add_effect(&mut self, effect: Effect<'a>) -> &mut Self {
        self.effects.push(effect);
        self
    }

    pub fn next(&self) -> Option<&Expression<'a>> {
        self.next.as_ref()
    }

    pub fn set_next(&mut self, next: Expression<'a>) -> &mut Self {
        self.next = Some(next);
        self
    }
}

#[derive(Debug)]
pub enum Effect<'a> {
    Flip,
    Scale(Expression<'a>),
    Spawn(Expression<'a>),
    Withdraw,
}
