use super::{
    combinator::{brackets, padding},
    expression::expression,
    IResult, Span,
};
use crate::ast::{
    expression::{Expression, Observable},
    state::{Effect, State, Transition},
};
use nom::{branch::alt, bytes::complete::tag};
use std::rc::Rc;

// Precedence levels.
pub fn contract(input: Span) -> IResult<Span, State> {
    padding(alt((or, disjunct)))(input)
}

pub fn disjunct(input: Span) -> IResult<Span, State> {
    padding(alt((and, conjunct)))(input)
}

pub fn conjunct(input: Span) -> IResult<Span, State> {
    padding(alt((unary, nullary)))(input)
}

pub fn unary(input: Span) -> IResult<Span, State> {
    padding(alt((brackets(contract), give, anytime, scale)))(input)
}

pub fn nullary(input: Span) -> IResult<Span, State> {
    padding(alt((zero, one)))(input)
}

// Contract combinators.
pub fn zero(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("zero")(input)?;
    Ok((input, State::default()))
}

pub fn one(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("one")(input)?;

    Ok((input, build_one_state()))
}

pub fn give(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("give")(input)?;
    let (input, next) = contract(input)?;

    Ok((input, build_give_state(next)))
}

pub fn and(input: Span) -> IResult<Span, State> {
    let (input, left) = conjunct(input)?;
    let (input, _) = tag("and")(input)?;
    let (input, right) = disjunct(input)?;

    Ok((input, build_and_state(left, right)))
}

pub fn or(input: Span) -> IResult<Span, State> {
    let (input, left) = disjunct(input)?;
    let (input, _) = tag("or")(input)?;
    let (input, right) = contract(input)?;

    Ok((input, build_or_state(left, right)))
}

pub fn anytime(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("anytime")(input)?;
    let (input, next) = contract(input)?;

    Ok((input, build_anytime_state(next)))
}

// TODO: semantic checks on Expression.
pub fn scale(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("scale")(input)?;
    let (input, scalar) = expression(input)?;
    let (input, sub_contract) = contract(input)?;

    Ok((input, build_scale_state(scalar, sub_contract)))
}

// Build state helper functions.
pub fn build_one_state() -> State {
    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    state
}

pub fn build_give_state(sub_contract: State) -> State {
    let mut transition = Transition::default();
    transition
        .add_effect(Effect::Flip)
        .set_next(sub_contract.into());

    let mut state = State::default();
    state.add_transition(transition);

    state
}

pub fn build_and_state(left: State, right: State) -> State {
    let mut transition = Transition::default();
    transition
        .add_effect(Effect::Spawn(right.into()))
        .set_next(left.into());

    let mut state = State::default();
    state.add_transition(transition);

    state
}

pub fn build_or_state(left: State, right: State) -> State {
    let is_holder = Rc::new(Expression::from(Observable::IsHolder));

    let mut left_transition = Transition::default();
    left_transition
        .add_condition(is_holder.clone())
        .set_next(left.into());

    let mut right_transition = Transition::default();
    right_transition
        .add_condition(is_holder.clone())
        .set_next(right.into());

    let mut state = State::default();
    state
        .add_transition(left_transition)
        .add_transition(right_transition);

    state
}

pub fn build_anytime_state(sub_contract: State) -> State {
    let mut transition = Transition::default();
    transition
        .add_condition(Expression::from(Observable::IsHolder).into())
        .set_next(sub_contract.into());

    let mut state = State::default();
    state.add_transition(transition);

    state
}

pub fn build_scale_state(factor: Expression, sub_contract: State) -> State {
    let mut transition = Transition::default();
    transition
        .add_effect(Effect::Scale(Rc::new(factor)))
        .set_next(sub_contract.into());

    let mut state = State::default();
    state.add_transition(transition);

    state
}

#[cfg(test)]
mod tests {
    use super::super::combinator::span;
    use super::*;
    use indoc::indoc;
    use nom::combinator::all_consuming;

    fn parse_contract_ok(input: &str, expected: (&str, State)) {
        assert_eq!(span(contract)(input), Ok(expected));
    }

    fn parse_contract_err(input: &str) {
        assert!(span(all_consuming(contract))(input).is_err());
    }

    #[test]
    fn parse_zero() {
        parse_contract_ok("zero", ("", State::default()));
    }

    #[test]
    fn parse_one() {
        parse_contract_ok("one", ("", build_one_state()));
    }

    #[test]
    fn parse_two() {
        parse_contract_err("two");
    }

    #[test]
    fn parse_give() {
        parse_contract_ok("give zero", ("", build_give_state(State::default())));

        parse_contract_ok(
            "give give zero",
            ("", build_give_state(build_give_state(State::default()))),
        );
    }

    #[test]
    fn parse_give_with_binary_operators() {
        // Equivalent to 'zero or (give zero)'.
        parse_contract_ok(
            "zero or give zero",
            (
                "",
                build_or_state(State::default(), build_give_state(State::default())),
            ),
        );

        // 'give' has lower precedence than 'or' so without brackets the input is equivalent to
        // 'give (zero or zero)'.
        parse_contract_ok(
            "give zero or zero",
            (
                "",
                build_give_state(build_or_state(State::default(), State::default())),
            ),
        );

        // Use brackets to enforce precedence.
        parse_contract_ok(
            "(give zero) or zero",
            (
                "",
                build_or_state(build_give_state(State::default()), State::default()),
            ),
        );
    }

    #[test]
    fn parse_and() {
        parse_contract_ok(
            "zero and one",
            ("", build_and_state(State::default(), build_one_state())),
        );

        parse_contract_ok(
            "zero and one and zero",
            (
                "",
                build_and_state(
                    State::default(),
                    build_and_state(build_one_state(), State::default()),
                ),
            ),
        );

        parse_contract_err("and");
        parse_contract_err("zero and");
        parse_contract_err("zero and one zero");
    }

    #[test]
    fn parse_or() {
        parse_contract_ok(
            "zero or one",
            ("", build_or_state(State::default(), build_one_state())),
        );

        parse_contract_ok(
            "zero or one or zero",
            (
                "",
                build_or_state(
                    State::default(),
                    build_or_state(build_one_state(), State::default()),
                ),
            ),
        );

        parse_contract_err("or");
        parse_contract_err("zero or");
        parse_contract_err("zero or one zero");
    }

    #[test]
    fn parse_anytime() {
        parse_contract_ok("anytime zero", ("", build_anytime_state(State::default())));

        parse_contract_ok(
            "anytime (zero or give zero)",
            (
                "",
                build_anytime_state(build_or_state(
                    State::default(),
                    build_give_state(State::default()),
                )),
            ),
        );

        parse_contract_ok(
            "anytime give zero",
            ("", build_anytime_state(build_give_state(State::default()))),
        );
    }

    #[test]
    fn parse_anytime_with_binary_operators() {
        // Brackets to enforce precedence.
        parse_contract_ok(
            "(anytime zero) or (give zero)",
            (
                "",
                build_or_state(
                    build_anytime_state(State::default()),
                    build_give_state(State::default()),
                ),
            ),
        );
    }

    #[test]
    fn parse_contract_with_padding_and_brackets() {
        parse_contract_ok(" (zero) ", ("", State::default()));
        parse_contract_ok("( zero )", ("", State::default()));
        parse_contract_ok(" ( zero ) ", ("", State::default()));
        parse_contract_ok(" ( (zero) ) ", ("", State::default()));
        parse_contract_ok(" ( (zero))", ("", State::default()));
    }

    #[test]
    fn parse_infix_contract_with_brackets() {
        parse_contract_ok(
            "(zero) or (zero)",
            ("", build_or_state(State::default(), State::default())),
        );
        parse_contract_ok(
            "((zero) or (zero))",
            ("", build_or_state(State::default(), State::default())),
        );
        parse_contract_ok(
            "zero or (zero or zero)",
            (
                "",
                build_or_state(
                    State::default(),
                    build_or_state(State::default(), State::default()),
                ),
            ),
        );
        parse_contract_ok(
            "(zero or zero) or zero",
            (
                "",
                build_or_state(
                    build_or_state(State::default(), State::default()),
                    State::default(),
                ),
            ),
        );
        parse_contract_ok(
            "(zero or zero) or (zero or zero)",
            (
                "",
                build_or_state(
                    build_or_state(State::default(), State::default()),
                    build_or_state(State::default(), State::default()),
                ),
            ),
        );
    }

    #[test]
    fn parse_scale() {
        let konst = Expression::Observable(Observable::Konst(Expression::Word(123).into()));
        parse_contract_ok(
            "scale (konst 123) zero",
            ("", build_scale_state(konst, State::default())),
        );
    }

    #[test]
    fn build_one() {
        let actual = format!("{:#?}", build_one_state());

        let expected = indoc!(
            "State {
                transitions: [
                    Transition {
                        conditions: [],
                        effects: [
                            Withdraw,
                        ],
                        next: None,
                    },
                ],
            }"
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_give() {
        let actual = format!("{:#?}", build_give_state(State::default()));

        let expected = indoc!(
            "State {
                transitions: [
                    Transition {
                        conditions: [],
                        effects: [
                            Flip,
                        ],
                        next: Some(
                            State {
                                transitions: [],
                            },
                        ),
                    },
                ],
            }"
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_and() {
        let actual = format!("{:#?}", build_and_state(State::default(), State::default()));

        let expected = indoc!(
            "State {
                transitions: [
                    Transition {
                        conditions: [],
                        effects: [
                            Spawn(
                                State {
                                    transitions: [],
                                },
                            ),
                        ],
                        next: Some(
                            State {
                                transitions: [],
                            },
                        ),
                    },
                ],
            }"
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_or() {
        let actual = format!("{:#?}", build_or_state(State::default(), State::default()));

        let expected = indoc!(
            "State {
                transitions: [
                    Transition {
                        conditions: [
                            Observable(
                                IsHolder,
                            ),
                        ],
                        effects: [],
                        next: Some(
                            State {
                                transitions: [],
                            },
                        ),
                    },
                    Transition {
                        conditions: [
                            Observable(
                                IsHolder,
                            ),
                        ],
                        effects: [],
                        next: Some(
                            State {
                                transitions: [],
                            },
                        ),
                    },
                ],
            }"
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_anytime() {
        let actual = format!("{:#?}", build_anytime_state(State::default()));

        let expected = indoc!(
            "State {
                transitions: [
                    Transition {
                        conditions: [
                            Observable(
                                IsHolder,
                            ),
                        ],
                        effects: [],
                        next: Some(
                            State {
                                transitions: [],
                            },
                        ),
                    },
                ],
            }"
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn build_scale() {
        let konst = Expression::Observable(Observable::Konst(Expression::Word(123).into()));
        let actual = format!("{:#?}", build_scale_state(konst, State::default()));

        let expected = indoc!(
            "State {
                transitions: [
                    Transition {
                        conditions: [],
                        effects: [
                            Scale(
                                Observable(
                                    Konst(
                                        Word(
                                            123,
                                        ),
                                    ),
                                ),
                            ),
                        ],
                        next: Some(
                            State {
                                transitions: [],
                            },
                        ),
                    },
                ],
            }"
        );

        assert_eq!(actual, expected);
    }
}
