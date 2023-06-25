use rocksdb::Error as RocksDBError;
use rocksdb::DB;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;

use async_trait::async_trait;

use crate::KVStore;
use crate::KeyString;

#[derive(Debug)]
#[allow(dead_code)]
pub struct RocksDBKVStore {
    db: rocksdb::DB,
}
impl RocksDBKVStore {
    pub fn new(path: &str) -> Result<Self, RocksDBError> {
        match DB::open_default(path) {
            Ok(db) => Ok(Self { db }),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl KVStore for RocksDBKVStore {
    async fn put<K, V>(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Send,
        V: Serialize + Send,
    {
        let key = key.as_str();
        let value = serde_json::to_string(&value)?;
        match self.db.put(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get<K, V>(&self, key: &K) -> Result<Option<V>, Box<dyn Error>>
    where
        K: KeyString + Sync,
        V: DeserializeOwned,
    {
        let key = key.as_str();
        match self.db.get(key) {
            Ok(value) => match value {
                None => Ok(None),
                Some(v) => {
                    let v = serde_json::from_slice(&v)?;
                    Ok(Some(v))
                }
            },
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn exists<K>(&self, key: &K) -> Result<bool, Box<dyn Error>>
    where
        K: KeyString + Sync,
    {
        let key = key.as_str();
        Ok(self.db.key_may_exist(key))
    }

    async fn delete<K>(&mut self, key: &K) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Sync,
    {
        let key = key.as_str();
        match self.db.delete(key) {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
