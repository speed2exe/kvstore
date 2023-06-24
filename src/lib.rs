use async_trait::async_trait;
use std::{error::Error, hash::Hash};

#[async_trait]
pub trait KVStore<K, V>
where
    K: Eq + Hash,
{
    async fn put(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>>;
    async fn get(&self, key: &K) -> Result<Option<V>, Box<dyn Error>>;
    async fn delete(&mut self, key: &K) -> Result<(), Box<dyn Error>>;
    async fn exists(&self, key: &K) -> Result<bool, Box<dyn Error>>;
}

pub mod mem_kv;
