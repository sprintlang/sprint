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
            Contract::And(first_contract, second_contract) => {
                self.visit_and(first_contract.as_ref(), second_contract.as_ref())
            }
            Contract::Or(first_contract, second_contract) => {
                self.visit_or(first_contract.as_ref(), second_contract.as_ref())
            }
        }
    }

    fn visit_zero(&mut self);

    fn visit_one(&mut self);

    fn visit_give(&mut self, contract: &Contract);

    fn visit_and(&mut self, first_contract: &Contract, second_contract: &Contract);

    fn visit_or(&mut self, first_contract: &Contract, second_contract: &Contract);
}
