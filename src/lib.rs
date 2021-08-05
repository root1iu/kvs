// empty
pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&self, k: String, v: String) {}

    pub fn remove(&self, k: String) {}

    pub fn get(&self, k: String) -> Option<String> {
        Some(String::from(""))
    }
}
