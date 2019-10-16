#[derive(PartialEq, Debug)]
pub enum Contract {
    Zero,
    One,
    Give(Box<Contract>),
    And(Box<Contract>, Box<Contract>),
    Or(Box<Contract>, Box<Contract>),
}

pub trait Visitor {
    fn visit(&mut self, contract: &Contract) {
        match contract {
            Contract::Zero => self.visit_zero(),
            Contract::One => self.visit_one(),
            Contract::Give(contract) => self.visit_give(contract.as_ref()),
            Contract::And(left, right) => self.visit_and(left.as_ref(), right.as_ref()),
            Contract::Or(left, right) => self.visit_or(left.as_ref(), right.as_ref()),
        }
    }

    fn visit_zero(&mut self);

    fn visit_one(&mut self);

    fn visit_give(&mut self, contract: &Contract);

    fn visit_and(&mut self, left: &Contract, right: &Contract);

    fn visit_or(&mut self, left: &Contract, right: &Contract);
}
