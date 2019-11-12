use super::{super::variable::Variable, Action};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

const DEPENDENCIES: &[&str] = &["0x0.LibraAccount", "0x0.LibraCoin"];

#[allow(dead_code)]
pub struct Deposit {
    amount: u64,
}

impl Deposit {
    #[allow(dead_code)]
    pub fn new(amount: u64) -> Self {
        Deposit { amount }
    }
}

impl Action for Deposit {
    fn dependencies(&self) -> &'static [&'static str] {
        DEPENDENCIES
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for Deposit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraCoin.deposit(
                Vector.borrow_mut<LibraCoin.T>(
                    &mut copy(contract_ref).coin_stores,
                    *(&copy(context_ref).coin_store_index),
                ), LibraAccount.withdraw_from_sender({}));",
            self.amount
        )
    }
}

pub struct Withdraw {
    payee: Address,
}

impl Withdraw {
    pub fn new(payee: Address) -> Self {
        Withdraw { payee }
    }
}

impl Action for Withdraw {
    fn dependencies(&self) -> &'static [&'static str] {
        DEPENDENCIES
    }

    fn properties(&self) -> Vec<Rc<Variable>> {
        vec![]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for Withdraw {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraAccount.deposit(
                {},
                LibraCoin.withdraw(
                    Vector.borrow_mut<LibraCoin.T>(
                        &mut copy(contract_ref).coin_stores,
                        *(&copy(context_ref).coin_store_index),
                    ),
                    *(&mut copy(context_ref).scale)
                )
            );",
            self.payee,
        )
    }
}

pub enum Address {
    Party,
    Counterparty,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Address::Party => write!(f, "*(&copy(context_ref).party)"),
            Address::Counterparty => write!(f, "*(&copy(context_ref).counterparty)"),
        }
    }
}
