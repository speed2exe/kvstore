use std::collections::HashMap;
use std::hash::Hash;

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

impl<K, V> KVStore<K, V> for MemKVStore<K, V>
where
    K: Eq + Hash,
{
    fn put(&mut self, key: K, value: V) -> Result<(), KVStoreError> {
        self.map.insert(key, value);
        Ok(())
    }

    fn get(&self, key: &K) -> Result<Option<&V>, KVStoreError> {
        let v = self.map.get(key);
        Ok(v)
    }

    fn delete(&mut self, key: &K) -> Result<(), KVStoreError> {
        let _value_at_key: Option<V> = self.map.remove(key);
        Ok(())
    }

    fn exists(&self, key: &K) -> Result<bool, KVStoreError> {
        Ok(self.map.contains_key(key))
    }
}
