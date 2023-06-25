use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::marker::Send;
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::{KVStore, KeyString};

#[derive(Debug)]
pub struct MemKVStore {
    map: RwLock<HashMap<String, String>>,
}

impl MemKVStore {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl KVStore for MemKVStore {
    async fn put<K, V>(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Send,
        V: Serialize + Send,
    {
        let key = key.as_str();
        let value = serde_json::to_string(&value)?;
        match self.map.try_write() {
            Ok(mut m) => {
                m.insert(key, value);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn get<K, V>(&self, key: &K) -> Result<Option<V>, Box<dyn Error>>
    where
        K: KeyString + Sync,
        V: DeserializeOwned,
    {
        let key = key.as_str();
        match self.map.try_read() {
            Ok(m) => {
                let value = m.get(&key);
                match value {
                    None => Ok(None),
                    Some(v) => {
                        let v = serde_json::from_str(v)?;
                        Ok(Some(v))
                    }
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn exists<K>(&self, key: &K) -> Result<bool, Box<dyn Error>>
    where
        K: KeyString + Sync,
    {
        let key = key.as_str();
        match self.map.try_read() {
            Ok(m) => Ok(m.contains_key(&key)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn delete<K>(&mut self, key: &K) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Sync,
    {
        let key = key.as_str();
        match self.map.try_write() {
            Ok(mut m) => {
                m.remove(&key);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
