use super::Expression;
use std::rc::Rc;

#[derive(Default, Debug, Eq, PartialEq)]
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

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Transition<'a> {
    conditions: Vec<Rc<Expression<'a>>>,
    effects: Vec<Effect<'a>>,
    next: Option<Rc<State<'a>>>,
}

impl<'a> Transition<'a> {
    pub fn conditions(&self) -> &[Rc<Expression>] {
        &self.conditions
    }

    pub fn add_condition(&mut self, condition: Rc<Expression<'a>>) -> &mut Self {
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

    pub fn next(&self) -> Option<Rc<State<'a>>> {
        self.next.clone()
    }

    pub fn set_next(&mut self, next: Rc<State<'a>>) -> &mut Self {
        self.next = Some(next);
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Effect<'a> {
    Flip,
    Scale(Rc<Expression<'a>>),
    Spawn(Rc<State<'a>>),
    Withdraw,
}
