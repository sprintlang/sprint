use super::{
    super::{expression::Address, expression::Expression, variable::Variable},
    Action,
};
use std::fmt::{self, Display, Formatter};

const DEPENDENCIES: &[&str] = &["0x0.LibraAccount", "0x0.LibraCoin"];

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

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for Deposit {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraCoin.deposit(
                Vector.borrow_mut<LibraCoin.T>(
                    &mut copy(contract_ref).coinstores,
                    *(&copy(context_ref).coinstore_index),
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

    fn definitions(&self) -> Vec<&Variable> {
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
                        &mut copy(contract_ref).coinstores,
                        *(&copy(context_ref).coinstore_index),
                    ),
                    *(&mut copy(context_ref).scale)
                )
            );",
            self.payee,
        )
    }
}

pub struct Emit<'a> {
    emitted_data: &'a Expression,
}

impl Action for Emit {
    fn dependencies(&self) -> &'static [&'static str] {
        &["0x0.LibraAccount"]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}
