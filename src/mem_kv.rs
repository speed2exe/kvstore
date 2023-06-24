use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::marker::Send;
use tokio::sync::RwLock;

use async_trait::async_trait;

use crate::KVStore;

#[derive(Debug)]
pub struct MemKVStore<K, V> {
    map: RwLock<HashMap<K, V>>,
}

impl<K, V> MemKVStore<K, V> {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl<K, V> KVStore<K, V> for MemKVStore<K, V>
where
    K: Eq + Hash + Send + Sync,
    V: Clone + Send + Sync,
{
    async fn put(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>> {
        match self.map.try_write() {
            Ok(mut m) => {
                m.insert(key, value);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn get(&self, key: &K) -> Result<Option<V>, Box<dyn Error>> {
        match self.map.try_read() {
            Ok(m) => {
                let value = m.get(key);
                let copy = value.cloned();
                Ok(copy)
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn exists(&self, key: &K) -> Result<bool, Box<dyn Error>> {
        match self.map.try_read() {
            Ok(m) => Ok(m.contains_key(key)),
            Err(e) => Err(Box::new(e)),
        }
    }
    async fn delete(&mut self, key: &K) -> Result<(), Box<dyn Error>> {
        match self.map.try_write() {
            Ok(mut m) => {
                m.remove(key);
                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
