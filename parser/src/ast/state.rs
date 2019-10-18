use super::expression::{kind::Observable, Expression};
use std::{cell::RefCell, rc::Rc};

#[derive(Default, Debug)]
pub struct State {
    transitions: Vec<Transition>,
}

impl State {
    pub fn transitions(&self) -> &[Transition] {
        &self.transitions
    }

    pub fn transitions_mut(&mut self) -> &mut [Transition] {
        &mut self.transitions
    }

    pub fn add_transition(&mut self, transition: Transition) -> &mut Self {
        self.transitions.push(transition);
        self
    }
}

#[derive(Default, Debug)]
pub struct Transition {
    conditions: Vec<Rc<dyn Expression>>,
    effects: Vec<Effect>,
    next: Option<Rc<RefCell<State>>>,
}

impl Transition {
    pub fn conditions(&self) -> &[Rc<dyn Expression>] {
        &self.conditions
    }

    pub fn add_condition(&mut self, condition: Rc<dyn Expression>) -> &mut Self {
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

    pub fn next(&self) -> Option<Rc<RefCell<State>>> {
        self.next.clone()
    }

    pub fn set_next(&mut self, next: Rc<RefCell<State>>) -> &mut Self {
        self.next = Some(next);
        self
    }
}

#[derive(Debug)]
pub enum Effect {
    Flip,
    Scale(Rc<Observable>),
    Withdraw,
}
