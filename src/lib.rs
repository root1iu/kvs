#![deny(missing_docs)]
//! kvs is an in-memory key/value store
extern crate serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::result;

extern crate failure;
#[macro_use]
extern crate failure_derive;

/// the alias of result::Result
pub type Result<T> = result::Result<T, ErrorType>;

/// opt data
#[derive(Serialize, Deserialize)]
enum OptData {
    SetData { key: String, value: String },
    RmData { key: String },
    GetData { key: String },
}

/// ErrorType
#[derive(Fail, Debug)]
#[fail(display = "My ErrorType")]
pub enum ErrorType {
    #[fail(display = "the key \"{}\" is nonexistent", _0)]
    /// nonexistent error
    Nonexistent(String),
    #[fail(display = "open \"{}\" fail for {}", _0, _1)]
    /// nonexistent error
    OpenFileFail(String, io::Error),
    #[fail(display = "log file fail")]
    /// log file fail
    LogFail(),
}

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
    pub fn set(&mut self, k: String, v: String) -> Result<String> {
        match self.kvs.insert(k, v) {
            Some(v) => Ok(String::from(v)),
            None => Ok(String::from("")),
        }
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
    pub fn remove(&mut self, k: String) -> Result<String> {
        match self.kvs.remove(&k) {
            Some(v) => Ok(String::from(v)),
            None => Err(ErrorType::Nonexistent(String::from(&k))),
        }
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
            None => Err(ErrorType::Nonexistent(String::from(&k))),
        }
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut pathbuf = path.into();
        pathbuf.push("kvs.db");
        match File::open(&pathbuf) {
            Ok(file) => {
                let mut kvs = KvStore::new();
                for line in io::BufReader::new(file).lines() {
                    match line {
                        Ok(line) => {
                            let decode: OptData = serde_json::from_str(&line).unwrap();
                            match decode {
                                OptData::SetData { key, value } => {
                                    let _ = kvs.set(key, value);
                                }
                                OptData::RmData { key } => {
                                    let _ = kvs.remove(key);
                                }
                                _ => {}
                            };
                        }
                        Err(_) => {}
                    }
                }
                Ok(kvs)
            }
            Err(err) => {
                return Err(ErrorType::OpenFileFail(
                    String::from(pathbuf.as_path().display().to_string()),
                    err,
                ))
            }
        }
    }
}
