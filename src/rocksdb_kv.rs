use rocksdb::Error as RocksDBError;
use rocksdb::DB;
use std::error::Error;
use std::hash::Hash;
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::KVStore;

#[derive(Debug)]
pub struct RocksDBKVStore {
    db: RwLock<DB>,
    // db: RwLock<DB>,
    // _phantom: PhantomData<K>,
    // _phantom: PhantomData<V>,
}

impl RocksDBKVStore {
    pub fn new(path: &str) -> Result<Self, RocksDBError> {
        match DB::open_default(path) {
            Ok(db) => Ok(Self {
                db: RwLock::new(db),
            }),
            Err(e) => Err(e),
        }
    }
}

// #[async_trait]
// impl<K, V> KVStore<K, V> for RocksDBKVStore
// where
//     K: Eq + Hash + Send + Sync,
//     V: Send + Sync,
// {
//     async fn put(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>> {
//         todo!();
//     }
//     async fn get(&self, key: &K) -> Result<Option<V>, Box<dyn Error>> {
//         todo!();
//     }
//     async fn delete(&mut self, key: &K) -> Result<(), Box<dyn Error>> {
//         todo!();
//     }
//     async fn exists(&self, key: &K) -> Result<bool, Box<dyn Error>> {
//         todo!();
//     }
// }
