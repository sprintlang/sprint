#[derive(PartialEq, Debug)]
pub enum Contract {
    Zero,
    One,
    Give(Box<Contract>),
}

pub trait Visitor {
    fn visit(&mut self, contract: &Contract) {
        match contract {
            Contract::Zero => self.visit_zero(),
            Contract::One => self.visit_one(),
            Contract::Give(_) => self.visit_give(),
        }
    }

    fn visit_zero(&mut self);

    fn visit_one(&mut self);

    fn visit_give(&mut self);
}
