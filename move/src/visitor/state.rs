use super::{definition::TERMINAL_ID, expression};
use crate::{
    jog::{
        action::{
            assert::Assert,
            flip::Flip,
            libra::{Address, Withdraw},
            scale::Scale,
            spawn::{PushContext, Spawn},
            update_state::UpdateState,
        },
        method::Transition,
        module,
        variable::Variable,
    },
    numbers::Numbers,
};
use sprint_parser::ast;
use std::{collections::HashMap, rc::Rc};

pub fn visit<'a>(state: &ast::state::State<'a>, numbers: &mut Numbers) -> module::Contract<'a> {
    let mut s = State::new(numbers);
    s.visit(state);

    s.into()
}

struct State<'a, 'b> {
    contract: module::Contract<'a>,
    ids: HashMap<*const ast::state::State<'a>, usize>,
    numbers: &'b mut Numbers,
}

impl<'a, 'b> State<'a, 'b> {
    fn new(numbers: &'b mut Numbers) -> Self {
        Self {
            contract: Default::default(),
            ids: Default::default(),
            numbers,
        }
    }

    fn visit(&mut self, state: &ast::state::State<'a>) -> usize {
        let key = state as *const _;

        if let Some(&state_id) = self.ids.get(&key) {
            // Do not generate code for the same state twice!
            return state_id;
        }

        // Zero is reserved for the terminal state.
        let id = self.numbers.next().unwrap();
        self.ids.insert(key, id);

        for transition in state.transitions() {
            let next_id = match transition.next() {
                Some(next) => match next {
                    ast::Expression::State(s) => self.visit(&s),
                    _ => unreachable!(),
                },
                None => TERMINAL_ID,
            };

            let mut method = Transition::new(id, next_id);

            for condition in transition.conditions() {
                method.add_action(Assert::new(expression::visit(condition, self.numbers), 0));
            }

            let mut post_actions = Vec::new();
            for effect in transition.effects() {
                match effect {
                    ast::state::Effect::Flip => method.add_action(Flip::default()),
                    ast::state::Effect::Scale(scalar) => {
                        method.add_action(Scale::new(expression::visit(scalar, self.numbers)))
                    }
                    ast::state::Effect::Spawn(root_state) => {
                        // TODO: actually visit the state (I guess)
                        let root_state = match root_state {
                            ast::Expression::State(state) => state,
                            _ => unreachable!(),
                        };

                        let root_id = self.visit(root_state);
                        let context = Rc::new(Variable {
                            // TODO: Make this random name gen to allow multiple spawns
                            // in the same transition method
                            name: "spawned_context",
                            type_name: "Self.Context",
                            default: None,
                        });
                        method.add_action(Spawn::new(context.clone(), root_id));
                        post_actions.push(PushContext::new(context));
                    }
                    ast::state::Effect::Withdraw => {
                        method.add_action(Withdraw::new(Address::Holder))
                    }
                }
            }

            method.add_action(UpdateState::new(method.to_state()));
            for action in post_actions {
                method.add_action(action);
            }

            self.contract.add_method(method);
        }

        id
    }
}

impl<'a> From<State<'a, '_>> for module::Contract<'a> {
    fn from(state: State<'a, '_>) -> Self {
        state.contract
    }
}
