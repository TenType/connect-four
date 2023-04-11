use crate::{AREA, MAX_SCORE, MIN_SCORE};
use std::collections::HashMap;

/// Separates the base-3 keys of positions in a cache buffer.
///
/// This should be a value that is impossible to produce from any game state.
pub const BUFFER_DELIMIT: u32 = 1;

/// A cache associating keys and scores of previously-computed positions.
pub struct Cache {
    max_depth: u8,
    table: HashMap<u64, i8>,
}

impl Default for Cache {
    fn default() -> Self {
        Self {
            max_depth: AREA,
            table: HashMap::new(),
        }
    }
}

impl Cache {
    /// Creates an empty cache.
    pub fn new(max_depth: u8) -> Self {
        Self {
            max_depth,
            ..Self::default()
        }
    }

    /// Creates a cache from a vector of bytes, returning [`None`] if the bytes do not have the correct format described below.
    ///
    /// # Bytes Format
    /// - First byte: the maximum depth of the cache.
    /// - Remaining bytes: buffers of little-endian u32s, representing base-3 keys.
    /// Each buffer is delimited by [`BUFFER_DELIMIT`] and is associated with a score, starting from [`MIN_SCORE`] incrementing up to [`MAX_SCORE`].
    pub fn from_bytes(bytes: Vec<u8>) -> Option<Self> {
        let (max_depth, rest_bytes) = bytes.split_first()?;
        let mut cache = Self::new(*max_depth);

        let mut score = MIN_SCORE;
        for chunk in rest_bytes.chunks_exact(4) {
            let key3 = u32::from_le_bytes(chunk.try_into().unwrap());
            if key3 == BUFFER_DELIMIT {
                if score == MAX_SCORE {
                    return Some(cache);
                }
                score += 1;
            } else {
                cache.insert(key3.into(), score);
            }
        }

        None
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

    /// Returns the maximum depth of the cache for optimization purposes.
    pub fn max_depth(&self) -> u8 {
        self.max_depth
    }
}
