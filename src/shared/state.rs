use mongodb::Database;
use crate::cache::kvstore::KVStore;


pub struct AppState {
    pub db: Database,
    pub cache: KVStore
}

impl AppState {
    pub fn new(db: Database) -> Self {
        Self {
            db,
            cache: KVStore::default()
        }
    }
}