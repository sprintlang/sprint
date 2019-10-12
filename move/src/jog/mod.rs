mod indent;

use std::io;

pub mod contract_module;
pub mod transactions;

pub trait Template {
    fn write(&self, w: &mut impl io::Write);
}
