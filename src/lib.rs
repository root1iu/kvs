// empty
pub struct KvStore {}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {}
    }

    pub fn set(&self, k: String, v: String) {
        panic!("unimplemented");
    }

    pub fn remove(&self, k: String) {
        panic!("unimplemented");
    }

    pub fn get(&self, k: String) -> Option<String> {
        panic!("unimplemented");
        Some(String::from(""))
    }
}
