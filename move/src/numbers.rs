use std::iter::{repeat, Enumerate, Map, Repeat, Skip};

static DEFAULT_START: usize = 1;

#[allow(clippy::type_complexity)]
pub struct Numbers(Map<Skip<Enumerate<Repeat<()>>>, fn((usize, ())) -> usize>);

impl Default for Numbers {
    fn default() -> Self {
        Self::new(DEFAULT_START)
    }
}

impl Numbers {
    pub fn new(start: usize) -> Self {
        Self(repeat(()).enumerate().skip(start).map(|(i, _)| i))
    }

    pub fn next(&mut self) -> usize {
        self.0.next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers() {
        let mut numbers = Numbers::default();

        for i in DEFAULT_START..(DEFAULT_START + 10) {
            assert_eq!(numbers.next(), i);
        }
    }
}
