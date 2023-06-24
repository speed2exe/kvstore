use std::hash::Hash;
// use async_trait::async_trait;

#[derive(Debug)]
pub struct KVStoreError;

pub trait KVStore<K, V>
where
    K: Eq + Hash,
{
    fn put(&mut self, key: K, value: V) -> Result<(), KVStoreError>;
    fn get(&self, key: &K) -> Result<Option<&V>, KVStoreError>;
    fn delete(&mut self, key: &K) -> Result<(), KVStoreError>;
    fn exists(&self, key: &K) -> Result<bool, KVStoreError>;
}

pub mod mem_kv;
