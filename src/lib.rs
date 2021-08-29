#![deny(missing_docs)]
//! kvs is an in-memory key/value store
extern crate serde;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;
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

#[derive(Debug)]
struct OffsetLen {
    offset: usize, // the offset of serialized OptData in log file
    len: usize,    // the length of serialized OptData(include '\n')
}

/// KvStore store the key-value in HashMap
pub struct KvStore {
    kvs: HashMap<String, OffsetLen>,
    log: Option<File>, // the object of log file
    log_off: usize,    // current offset of log file
    log_name: PathBuf, // the name of log file
}

impl Default for KvStore {
    fn default() -> Self {
        match env::current_dir() {
            Ok(pathbuf) => match KvStore::open(pathbuf) {
                Ok(kv) => kv,
                Err(_) => KvStore {
                    kvs: HashMap::new(),
                    log: None,
                    log_off: 0,
                    log_name: PathBuf::new(),
                },
            },
            Err(_) => KvStore {
                kvs: HashMap::new(),
                log: None,
                log_off: 0,
                log_name: PathBuf::new(),
            },
        }
    }
}

impl KvStore {
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
        let data = OptData::SetData {
            key: String::from(&k),
            value: String::from(&v),
        };
        let mut offset: usize = self.log_off;
        match self.kvs.get(&k) {
            Some(off2len) => {
                offset = off2len.offset;
            }
            None => {}
        }
        let mut data_str = serde_json::to_string(&data)?;
        data_str.push('\n');
        let value = self.kvs.entry(k).or_insert(OffsetLen {
            offset: offset,
            len: data_str.len(),
        });
        if value.len != data_str.len() {
            self.rebuild_log(offset, &data_str)?;
        } else {
            match &mut self.log {
                Some(log) => {
                    log.write_at(data_str.as_bytes(), offset as u64)?;
                    if offset + data_str.len() > self.log_off {
                        self.log_off = offset + data_str.len();
                    }
                }
                None => {}
            }
        }
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
    pub fn remove(&mut self, k: String) -> Result<String> {
        let data = OptData::RmData {
            key: String::from(&k),
        };
        let mut data_str = serde_json::to_string(&data)?;
        match &mut self.log {
            Some(log) => {
                data_str.push('\n');
                log.write_all(data_str.as_bytes())?;
                self.log_off += data_str.len();
            }
            None => {}
        }
        match self.kvs.remove(&k) {
            Some(_) => Ok(String::from(&k)),
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
    pub fn get(&mut self, k: String) -> Result<Option<String>> {
        let off2len = match self.kvs.get(&k) {
            Some(v) => v,
            None => {
                return Ok(None);
            }
        };
        match &mut self.log {
            Some(log) => {
                let mut buf = String::new();
                log.seek(SeekFrom::Start(off2len.offset as u64))?;
                log.take(off2len.len as u64).read_to_string(&mut buf)?;
                let decode: OptData = serde_json::from_str(&buf)?;
                match decode {
                    OptData::SetData { key: _, value } => return Ok(Some(value)),
                    _ => return Ok(None),
                }
            }
            None => {}
        }
        Ok(None)
    }

    // create a temporary log file to rebuild the new log
    // then rename temporary log file to self.log_name
    fn rebuild_log(&mut self, offset: usize, data: &String) -> Result<()> {
        let mut sorted: Vec<_> = self.kvs.iter_mut().collect();
        sorted.sort_by(|l, r| l.1.offset.cmp(&r.1.offset));

        let mut new_path = PathBuf::from(&self.log_name);
        new_path.set_file_name("kvs.log.swp");
        let new_file = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&new_path)?;

        let mut new_file_offset: i32 = 0;
        let mut diff: i32 = 0;
        match &mut self.log {
            Some(log) => {
                for (_, value) in sorted.iter_mut() {
                    let mut buf = String::new();
                    if offset == value.offset {
                        if data.len() != value.len {
                            diff = data.len() as i32 - value.len as i32;
                        }
                        buf = String::from(data);
                    } else {
                        log.seek(SeekFrom::Start(value.offset as u64))?;
                        log.take(value.len as u64).read_to_string(&mut buf)?;
                        new_file_offset = value.offset as i32 + diff;
                    }
                    new_file.write_at(&buf.as_bytes(), new_file_offset as u64)?;
                    value.offset = new_file_offset as usize;
                    value.len = buf.len();
                    new_file_offset = value.offset as i32 + value.len as i32;
                }

                fs::rename(&new_path, &self.log_name)?;
                Ok(())
            }
            None => Ok(()),
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
            .open(&pathbuf)?;
        let mut kvs: HashMap<String, OffsetLen> = HashMap::new();
        let mut offset: usize = 0;
        for line in io::BufReader::new(&file).lines() {
            let line = line?;
            let decode: OptData = serde_json::from_str(&line)?;
            match decode {
                OptData::SetData { key, value: _ } => {
                    kvs.insert(
                        key,
                        OffsetLen {
                            offset,
                            len: line.len() + 1,
                        },
                    );
                }
                OptData::RmData { key } => {
                    kvs.remove(&key);
                }
                // ignore get
                _ => {}
            };
            offset += line.len() + 1;
        }
        Ok(KvStore {
            kvs,
            log: Some(file),
            log_off: offset,
            log_name: PathBuf::from(pathbuf),
        })
    }
}
