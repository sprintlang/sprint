use super::Expression;

#[derive(Default, Debug)]
pub struct State {
    transitions: Vec<Transition>,
}

impl State {
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    pub fn add_transition(&mut self, transition: Transition) -> &mut Self {
        self.transitions.push(transition);
        self
    }
}

#[derive(Default, Debug)]
pub struct Transition {
    conditions: Vec<Expression>,
    effects: Vec<Effect>,
    next: Option<Expression>,
}

impl Transition {
    pub fn conditions(&self) -> &[Expression] {
        &self.conditions
    }

    pub fn add_condition(&mut self, condition: Expression) -> &mut Self {
        self.conditions.push(condition);
        self
    }

    pub fn effects(&self) -> Vec<&Effect> {
        self.effects.iter().rev().collect()
    }

    pub fn add_effect(&mut self, effect: Effect) -> &mut Self {
        self.effects.push(effect);
        self
    }

    pub fn next(&self) -> Option<&Expression> {
        self.next.as_ref()
    }

    pub fn set_next(&mut self, next: Expression) -> &mut Self {
        self.next = Some(next);
        self
    }
}

#[derive(Debug)]
pub enum Effect {
    Flip,
    Scale(Expression),
    Spawn(Expression),
    Withdraw,
}
