use std::collections::HashMap;

/*
pub struct KVStore<'a, T> {
    items: HashMap<&'a str, T>
}

impl<'a, T> KVStore<'a, T> {
    pub fn insert(&mut self, item_name: &'a str, item: T) {
        self.items.insert(item_name, item);
    }

    pub fn get(&self, item_name: &'a str) -> Option<&T> {
        self.items.get(item_name)
    }
}*/

pub struct KVStore {
    items: HashMap<String, Vec<isize>>
}

impl Default for KVStore {
    fn default() -> Self {
        Self {
            items: HashMap::new()
        }
    }
}