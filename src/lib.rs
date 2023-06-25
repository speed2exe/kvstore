use std::error::Error;

use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait KVStore {
    async fn put<K, V>(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Send,
        V: Serialize + Send;

    async fn get<K, V>(&self, key: &K) -> Result<Option<V>, Box<dyn Error>>
    where
        K: KeyString + Sync,
        V: DeserializeOwned;

    async fn exists<K>(&self, key: &K) -> Result<bool, Box<dyn Error>>
    where
        K: KeyString + Sync;

    async fn delete<K>(&mut self, key: &K) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Sync;
}

pub trait KeyString {
    fn as_str(&self) -> String;
}

pub mod mem_kv;
pub mod rocksdb_kv;
