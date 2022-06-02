use std::collections::HashMap;

use crate::engine::d2::subsystem::StorageSystem;

use super::Dynamic;

pub struct DummyStorage {
    hash: HashMap<String, Dynamic>,
}

impl DummyStorage {
    pub fn new() -> Self {
        Self {
            hash: HashMap::new(),
        }
    }
}

impl StorageSystem for DummyStorage {
    fn is_supported(&self) -> bool {
        false
    }

    fn set<A>(&self, key: String, value: A) -> bool {
        // self.hash.insert(key, value);
        // true
        unimplemented!()
    }

    fn get<A>(&self, key: String) -> Option<&A> {
        // self.hash.get(&key)
        unimplemented!()
    }

    fn remove(&mut self, key: String) {
        self.hash.remove(&key);
    }

    fn clear(&mut self) {
        self.hash.clear();
    }
}
