use std::{cmp::Eq, collections::HashMap, hash::Hash, iter::IntoIterator};

#[derive(Clone, Debug)]
pub struct HashCount<T: Eq + Hash>(HashMap<T, usize>);

impl<T: Eq + Hash> Default for HashCount<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T: Eq + Hash> HashCount<T> {
    pub fn count(&self, value: &T) -> usize {
        self.0.get(value).copied().unwrap_or_default()
    }

    pub fn insert(&mut self, value: T) {
        let count = self.count(&value);
        self.0.insert(value, count + 1);
    }

    pub fn insert_without_increment(&mut self, value: T) {
        let count = self.count(&value);
        self.0.insert(value, count);
    }

    pub fn take(&mut self, value: &T) -> Option<T> {
        self.0.remove_entry(value).map(|(value, _)| value)
    }

    pub fn replace(&mut self, value: T, count: usize) -> Option<T> {
        match self.0.remove_entry(&value) {
            Some((old_value, old_count)) => {
                self.0.insert(value, count + old_count);
                Some(old_value)
            }
            None => {
                self.0.insert(value, count);
                None
            }
        }
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a HashCount<T> {
    type Item = (&'a T, &'a usize);
    type IntoIter = std::collections::hash_map::Iter<'a, T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

impl<T: Eq + Hash> IntoIterator for HashCount<T> {
    type Item = (T, usize);
    type IntoIter = std::collections::hash_map::IntoIter<T, usize>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
