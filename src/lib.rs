#![deny(missing_docs)]
//! kvs is an in-memory key/value store
use std::collections::HashMap;
use std::path::PathBuf;
use std::result;

extern crate failure;
#[macro_use]
extern crate failure_derive;

/// the alias of result::Result
pub type Result<T> = result::Result<T, ErrorType>;

/// ErrorType
#[derive(Fail, Debug)]
#[fail(display = "My ErrorType")]
pub struct ErrorType {}

/// KvStore store the key-value in HashMap
pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl Default for KvStore {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn set(&mut self, k: String, v: String) -> Result<()> {
        self.kvs.insert(k, v);
        Ok(())
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
    pub fn remove(&mut self, k: String) -> Result<()> {
        self.kvs.remove(&k);
        Ok(())
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
    pub fn get(&self, k: String) -> Result<Option<String>> {
        match self.kvs.get(&k) {
            Some(v) => Ok(Some(String::from(v))),
            None => Err(ErrorType {}),
        }
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        Err(ErrorType {})
    }
}
