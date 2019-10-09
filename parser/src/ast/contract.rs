#[derive(PartialEq, Debug)]
pub enum Contract {
    Zero,
    One,
}

pub trait Visitor {
    fn visit(&mut self, contract: &Contract) {
        match contract {
            Contract::Zero => self.visit_zero(),
            Contract::One => self.visit_one(),
        }
    }

    fn visit_zero(&mut self);

    fn visit_one(&mut self);
}
