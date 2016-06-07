use std::collections::HashMap;
use std::hash::Hash;

pub trait Unique {
    type Identifier: Eq + Hash;

    fn get_unique_identifier(&self) -> Self::Identifier;
}

pub struct DataStore<T: Unique> {
    data: HashMap<T::Identifier, T>,
}

impl<T: Unique> DataStore<T> {
    pub fn new() -> DataStore<T> {
        DataStore::<T> {
            data: HashMap::new()
        }
    }

    pub fn get(&self, key: &T::Identifier) -> Option<&T> {
        self.data.get(key)
    }

    pub fn has(&self, key: &T::Identifier) -> bool {
        self.data.contains_key(key)
    }

    pub fn add(&mut self, data: T) {
        self.data.insert(data.get_unique_identifier(), data);
    }
}
