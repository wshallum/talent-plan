#![deny(missing_docs)]

//! The kvs crate contains an in-memory Key-Value store (`kvs::KvStore`) and
//! a command line tool `kvs`.

use std::collections::HashMap;

/// An in-memory Key-Value store where the Keys and Values are both Strings.
#[derive(Default)]
pub struct KvStore {
    kv: HashMap<String, String>,
}

impl KvStore {
    /// Creates a new instance of the Key-Value store.
    /// ```
    /// let store = kvs::KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore { kv: HashMap::new() }
    }

    /// Gets a value from the store if one exists, otherwise returns None.
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// assert_eq!(store.get("key".to_owned()), None);
    /// store.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
    /// ```
    pub fn get(&self, k: String) -> Option<String> {
        self.kv.get(&k).map(|s| s.to_owned())
    }

    /// Stores a value in the store, overwriting existing values, if any.
    pub fn set(&mut self, k: String, v: String) {
        self.kv.insert(k, v);
    }

    /// Removes a value from the store.
    /// ```
    /// let mut store = kvs::KvStore::new();
    /// store.set("key".to_owned(), "value".to_owned());
    /// assert_eq!(store.get("key".to_owned()), Some("value".to_owned()));
    /// store.remove("key".to_owned());
    /// assert_eq!(store.get("key".to_owned()), None);
    /// ```
    pub fn remove(&mut self, k: String) {
        self.kv.remove(&k);
    }
}
