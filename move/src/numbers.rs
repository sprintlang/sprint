#[derive(Default, Debug)]
pub struct Numbers(u64);

impl Iterator for Numbers {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let result = Some(self.0);
        self.0 += 1;

        result
    }
}

impl From<u64> for Numbers {
    fn from(start: u64) -> Self {
        Self(start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let mut numbers = Numbers::default();

        for i in 0..10 {
            assert_eq!(numbers.next().unwrap(), i);
        }
    }
}
