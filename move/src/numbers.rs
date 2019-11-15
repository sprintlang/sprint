#[derive(Default)]
pub struct Numbers(usize);

impl Iterator for Numbers {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let result = Some(self.0);
        self.0 += 1;

        result
    }
}

impl From<usize> for Numbers {
    fn from(start: usize) -> Self {
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
