#![deny(missing_docs)]
//! kvs is an in-memory key/value store
use std::collections::HashMap;

/// KvStore store the key-value in HashMap
pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl KvStore {
    /// Creates an empty `KvStore`.
    ///
    /// The KvStore is initially created with HashMap
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    /// let mut store = KvStore::new();
    /// ```
    pub fn new() -> KvStore {
        KvStore {
            kvs: HashMap::new(),
        }
    }

    /// Inserts a key-value pair into the KvStore.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn set(&mut self, k: String, v: String) {
        self.kvs.insert(k, v);
    }

    /// Removes a key from the KvStore
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// store.remove("key1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), None);
    /// ```
    pub fn remove(&mut self, k: String) {
        self.kvs.remove(&k);
    }

    /// Returns a copy of the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use kvs::KvStore;
    ///
    /// let mut store = KvStore::new();
    ///
    /// store.set("key1".to_owned(), "value1".to_owned());
    /// assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    /// ```
    pub fn get(&self, k: String) -> Option<String> {
        match self.kvs.get(&k) {
            Some(v) => Some(String::from(v)),
            None => None,
        }
    }
}
