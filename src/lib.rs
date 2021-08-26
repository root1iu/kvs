#![deny(missing_docs)]
//! kvs is an in-memory key/value store
extern crate serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::result;

extern crate failure;
use failure::Error;

/// the alias of result::Result
pub type Result<T> = result::Result<T, Error>;

/// opt data
#[derive(Serialize, Deserialize, Debug)]
pub enum OptData {
    /// set data: key-value
    SetData {
        /// key
        key: String,
        /// value
        value: String,
    },
    /// remove data: key
    RmData {
        /// key
        key: String,
    },
    /// get data: key
    GetData {
        /// key
        key: String,
    },
}

/// KvStore store the key-value in HashMap
pub struct KvStore {
    /// KvStore store the key-value in HashMap
    kvs: HashMap<String, String>,
    log: Option<File>,
}

impl Default for KvStore {
    fn default() -> Self {
        match env::current_dir() {
            Ok(pathbuf) => match KvStore::open(pathbuf) {
                Ok(kv) => kv,
                Err(_) => KvStore {
                    kvs: HashMap::new(),
                    log: None,
                },
            },
            Err(_) => KvStore {
                kvs: HashMap::new(),
                log: None,
            },
        }
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
    // pub fn new() -> KvStore {
    //     KvStore {
    //         kvs: HashMap::new(),
    //         log: None,
    //     }
    // }

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
        let data = OptData::SetData {
            key: String::from(&k),
            value: String::from(&v),
        };
        let mut data_str = serde_json::to_string(&data)?;
        match &mut self.log {
            Some(log) => {
                data_str.push('\n');
                log.write_all(data_str.as_bytes())?;
            }
            None => {}
        }
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
        let data = OptData::RmData {
            key: String::from(&k),
        };
        let mut data_str = serde_json::to_string(&data)?;
        match &mut self.log {
            Some(log) => {
                data_str.push('\n');
                log.write_all(data_str.as_bytes())?;
            }
            None => {}
        }
        match self.kvs.remove(&k) {
            Some(v) => Ok(String::from(v)),
            None => Err(failure::format_err!("Key not found")),
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
            None => Ok(None),
        }
    }

    /// Open the KvStore at a given path. Return the KvStore.
    pub fn open(path: impl Into<PathBuf>) -> Result<KvStore> {
        let mut pathbuf = path.into();
        pathbuf.push("kvs.log");
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .append(true)
            .open(&pathbuf)?;
        let mut kvs: HashMap<String, String> = HashMap::new();
        for line in io::BufReader::new(&file).lines() {
            let line = line?;
            let decode: OptData = serde_json::from_str(&line)?;
            match decode {
                OptData::SetData { key, value } => {
                    kvs.insert(key, value);
                }
                OptData::RmData { key } => {
                    kvs.remove(&key);
                }
                // ignore get
                _ => {}
            };
        }
        Ok(KvStore {
            kvs,
            log: Some(file),
        })
    }
}
