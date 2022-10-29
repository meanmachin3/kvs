#![deny(missing_docs)]
//! The KvStore stores key/value pairs.
use std::collections::HashMap;

/// Key/value pairs are stored in `HashMap` in memory and not persisted to disk.
///
/// Usage:
/// ```
/// use kvs::KvStore;
///
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    /// Constructor for `KvStore`
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    /// Stores the value of a string key to a string
    /// In case of collision, previous value would be overwritten.
    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    /// Stores the value of a string key to a string
    ///
    /// Returns `None` in case the value does not exist in hashmap
    pub fn get(&self, key: String) -> Option<String> {
        self.map.get(&key).cloned()
    }

    /// Removes the key from the in-memory store.
    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
