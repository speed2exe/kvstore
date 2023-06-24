use async_trait::async_trait;
use std::hash::Hash;

#[derive(Debug)]
pub struct KVStoreError;

#[async_trait]
pub trait KVStore<K, V>
where
    K: Eq + Hash,
{
    async fn put(&mut self, key: K, value: V) -> Result<(), KVStoreError>;
    async fn get(&self, key: &K) -> Result<Option<&V>, KVStoreError>;
    async fn delete(&mut self, key: &K) -> Result<(), KVStoreError>;
    async fn exists(&self, key: &K) -> Result<bool, KVStoreError>;
}

pub mod mem_kv;
