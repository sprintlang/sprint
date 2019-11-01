use super::{super::variable::Variable, Action};
use std::{
    fmt::{self, Display, Formatter},
    rc::Rc,
};

const DEPENDENCIES: &[&str] = &["0x0.LibraAccount", "0x0.LibraCoin"];
const COIN_STORE: &str = "coin_store";

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
        vec![Rc::new(Variable {
            name: String::from(COIN_STORE),
            type_name: "LibraCoin.T",
            default: Some(String::from("LibraCoin.zero()")),
        })]
    }

    fn definitions(&self) -> Vec<Rc<Variable>> {
        vec![]
    }
}

impl Display for Deposit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraCoin.deposit(&mut move(contract_ref).{}, LibraAccount.withdraw_from_sender({}));",
            COIN_STORE, self.amount
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
            "LibraAccount.deposit({}, LibraCoin.withdraw(&mut copy(contract_ref).{}, *(&mut copy(context_ref).scale)));",
            self.payee, COIN_STORE
        )
    }
}

pub enum Address {
    Holder,
    #[allow(dead_code)]
    Counterparty,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Address::Holder => write!(f, "*(&copy(contract_ref).holder)"),
            Address::Counterparty => write!(f, "*(&copy(contract_ref).counterparty)"),
        }
    }
}
