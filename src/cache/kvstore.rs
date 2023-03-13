use std::collections::HashMap;

use crate::models::user::User;

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
    items: HashMap<String, Vec<User>>
}

impl Default for KVStore {
    fn default() -> Self {
        Self {
            items: HashMap::new()
        }
    }
}

impl KVStore {
    pub fn insert(&mut self, key: String, value: Vec<User>) {
        self.items.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<Vec<User>> {
        self.items.get(key).cloned()
    }
}