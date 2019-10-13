mod indent;

use std::io;

pub mod contract_module;
pub mod lock_libra_action;
pub mod unlock_libra_action;

pub trait Template {
    fn write(&self, w: &mut impl io::Write);
}
