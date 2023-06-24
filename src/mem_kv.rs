use std::collections::HashMap;
use std::hash::Hash;
use std::marker::Send;

use async_trait::async_trait;

use crate::KVStore;
use crate::KVStoreError;

#[derive(Debug)]
pub struct MemKVStore<K, V> {
    map: HashMap<K, V>,
}

impl<K, V> MemKVStore<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

#[async_trait]
impl<K, V> KVStore<K, V> for MemKVStore<K, V>
where
    K: Eq + Hash + Send + Sync,
    V: Send + Sync,
{
    async fn put(&mut self, key: K, value: V) -> Result<(), KVStoreError> {
        self.map.insert(key, value);
        Ok(())
    }
    async fn get(&self, key: &K) -> Result<Option<&V>, KVStoreError> {
        let v = self.map.get(key);
        Ok(v)
    }
    async fn exists(&self, key: &K) -> Result<bool, KVStoreError> {
        Ok(self.map.contains_key(key))
    }
    async fn delete(&mut self, key: &K) -> Result<(), KVStoreError> {
        let _value_at_key: Option<V> = self.map.remove(key);
        Ok(())
    }
}
