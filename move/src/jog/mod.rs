use std::io;

pub trait Template {
    fn write(&self, w: &mut impl io::Write);
}
