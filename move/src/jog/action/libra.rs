use super::{
    super::{
        expression::{Address, Expression},
        variable::{Variable, EVENT},
    },
    Action,
};
use std::fmt::{self, Display, Formatter};

const DEPENDENCIES: &[&str] = &["0x0.LibraAccount", "0x0.LibraCoin"];

#[derive(Debug)]
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

#[derive(Debug)]
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
                *(&{}),
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

#[derive(Debug)]
pub struct Emit<'a> {
    emitted_data: Expression<'a>,
}

impl<'a> Emit<'a> {
    pub fn new(emitted_data: Expression<'a>) -> Self {
        Emit { emitted_data }
    }
}

impl Action for Emit<'_> {
    fn dependencies(&self) -> &'static [&'static str] {
        &["0x0.LibraAccount"]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for Emit<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraAccount.emit_event<u64>(&mut {}, {});",
            EVENT.identifier(),
            self.emitted_data
        )
    }
}

#[derive(Debug)]
pub struct DestroyHandle;

impl Action for DestroyHandle {
    fn dependencies(&self) -> &'static [&'static str] {
        &["0x0.LibraAccount"]
    }

    fn definitions(&self) -> Vec<&Variable> {
        vec![]
    }
}

impl Display for DestroyHandle {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "LibraAccount.destroy_handle<u64>(move({}));",
            EVENT.identifier()
        )
    }
}
