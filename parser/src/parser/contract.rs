use super::{
    combinator::{brackets, padding},
    IResult, Span,
};
use crate::ast::{
    expression::{Expression, Observable},
    state::{Effect, State, Transition},
};
use nom::{branch::alt, bytes::complete::tag};
use std::rc::Rc;

// Precedence levels
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
    padding(alt((brackets(contract), give, anytime)))(input)
}

pub fn nullary(input: Span) -> IResult<Span, State> {
    padding(alt((zero, one)))(input)
}

// Contract combinators

pub fn zero(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("zero")(input)?;
    Ok((input, State::default()))
}

pub fn one(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("one")(input)?;

    Ok((input, build_one_state()))
}

pub fn build_one_state() -> State {
    let mut transition = Transition::default();
    transition.add_effect(Effect::Withdraw);

    let mut state = State::default();
    state.add_transition(transition);

    state
}

pub fn give(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("give")(input)?;
    let (input, next) = contract(input)?;

    Ok((input, build_give_state(next)))
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

pub fn and(input: Span) -> IResult<Span, State> {
    let (input, _left) = conjunct(input)?;
    let (input, _) = tag("and")(input)?;
    let (input, _right) = disjunct(input)?;

    // TODO: implement
    Ok((input, State::default()))
}

pub fn or(input: Span) -> IResult<Span, State> {
    let (input, left) = disjunct(input)?;
    let (input, _) = tag("or")(input)?;
    let (input, right) = contract(input)?;

    Ok((input, build_or_state(left, right)))
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

pub fn anytime(input: Span) -> IResult<Span, State> {
    let (input, _) = tag("anytime")(input)?;
    let (input, next) = contract(input)?;

    Ok((input, build_anytime_state(next)))
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

#[cfg(test)]
mod tests {
    use super::super::combinator::span;
    use super::*;
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
    fn build_one() {
        let actual_state = build_one_state();

        let mut transition = Transition::default();
        transition.add_effect(Effect::Withdraw);

        let mut expected_state = State::default();
        expected_state.add_transition(transition);

        assert_eq!(actual_state, expected_state);
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
    fn parse_contract_with_padding_and_brackets() {
        parse_contract_ok(" (zero) ", ("", State::default()));
        parse_contract_ok("( zero )", ("", State::default()));
        parse_contract_ok(" ( zero ) ", ("", State::default()));
        parse_contract_ok(" ( (zero) ) ", ("", State::default()));
        parse_contract_ok(" ( (zero))", ("", State::default()));
    }

    #[test]
    fn build_or() {
        let actual_state = build_or_state(State::default(), State::default());

        let is_holder = Rc::new(Expression::from(Observable::IsHolder));

        let mut left_transition = Transition::default();
        left_transition
            .add_condition(is_holder.clone())
            .set_next(State::default().into());

        let mut right_transition = Transition::default();
        right_transition
            .add_condition(is_holder.clone())
            .set_next(State::default().into());

        let mut expected_state = State::default();
        expected_state
            .add_transition(left_transition)
            .add_transition(right_transition);

        assert_eq!(actual_state, expected_state);
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
    fn build_give() {
        let actual_state = build_give_state(State::default());

        let mut transition = Transition::default();
        transition
            .add_effect(Effect::Flip)
            .set_next(State::default().into());

        let mut expected_state = State::default();
        expected_state.add_transition(transition);

        assert_eq!(actual_state, expected_state);
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
        // zero or (give zero).
        parse_contract_ok(
            "zero or give zero",
            (
                "",
                build_or_state(State::default(), build_give_state(State::default())),
            ),
        );

        // give has higher precedence or so without brackets is equivalent to
        // give (zero or zero).
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
    fn build_anytime() {
        let actual_state = build_anytime_state(State::default());

        let mut transition = Transition::default();
        transition
            .add_condition(Expression::from(Observable::IsHolder).into())
            .set_next(State::default().into());

        let mut expected_state = State::default();
        expected_state.add_transition(transition);

        assert_eq!(actual_state, expected_state);
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

    //     #[test]
    //     fn parse_and() {
    //         parse_contract_ok(
    //             "zero and zero",
    //             (
    //                 "",
    //                 Contract::And(Box::new(Contract::Zero), Box::new(Contract::Zero)),
    //             ),
    //         );

    //         // Right associative so without bracketing `zero and zero and zero`
    //         // is equivalent to `zero and (zero and zero)`.
    //         parse_contract_ok(
    //             "zero and zero and zero",
    //             (
    //                 "",
    //                 Contract::And(
    //                     Box::new(Contract::Zero),
    //                     Box::new(Contract::And(
    //                         Box::new(Contract::Zero),
    //                         Box::new(Contract::Zero),
    //                     )),
    //                 ),
    //             ),
    //         );

    //         parse_contract_ok(
    //             "zero and give zero",
    //             (
    //                 "",
    //                 Contract::And(
    //                     Box::new(Contract::Zero),
    //                     Box::new(Contract::Give(Box::new(Contract::Zero))),
    //                 ),
    //             ),
    //         );

    //         parse_contract_err("and");
    //         parse_contract_err("and zero");
    //         parse_contract_err("and zero zero");
    //     }

    //     #[test]
    //     fn parse_with_precedence() {
    //         parse_contract_ok(
    //             "zero and one or zero",
    //             (
    //                 "",
    //                 Contract::Or(
    //                     Box::new(Contract::And(
    //                         Box::new(Contract::Zero),
    //                         Box::new(Contract::One),
    //                     )),
    //                     Box::new(Contract::Zero),
    //                 ),
    //             ),
    //         );
    //     }
}
