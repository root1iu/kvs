use std::collections::HashMap;

pub struct KvStore {
    kvs: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            kvs: HashMap::new(),
        }
    }

    pub fn set(&mut self, k: String, v: String) {
        self.kvs.insert(k, v);
    }

    pub fn remove(&mut self, k: String) {
        self.kvs.remove(&k);
    }

    pub fn get(&self, k: String) -> Option<String> {
        match self.kvs.get(&k) {
            Some(v) => Some(String::from(v)),
            None => None,
        }
    }
}
