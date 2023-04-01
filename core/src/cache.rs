use std::collections::HashMap;

/// A transposition table used to cache the scores of previously-computed positions.
#[derive(Default)]
pub struct Cache {
    table: HashMap<u64, i8>,
}

impl Cache {
    /// Creates an empty cache.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the score of a given position's key or [`None`](Option::None) if the key does not exist in the cache.
    pub fn get(&self, key: &u64) -> Option<i8> {
        self.table.get(key).copied()
    }

    /// Inserts a position's key and its score as a key-value pair into the cache.
    pub fn insert(&mut self, key: u64, value: i8) {
        self.table.insert(key, value);
    }

    /// Clears the cache, removing all key-value pairs.
    pub fn clear(&mut self) {
        self.table.clear();
    }

    /// Returns `true` if the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    /// Returns the number of elements in the cache.
    pub fn len(&self) -> usize {
        self.table.len()
    }
}
