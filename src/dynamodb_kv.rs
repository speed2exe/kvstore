use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::Client;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;

use async_trait::async_trait;

use crate::KVStore;
use crate::KeyString;

#[derive(Debug)]
#[allow(dead_code)]
pub struct DynamoDBKVStore {
    client: Client,
    table_name: String,
    partition_key: String,
    value_key: String,
}
impl DynamoDBKVStore {
    pub fn new(
        client: Client,
        table_name: String,
        partition_key: String,
        value_key: String,
    ) -> Self {
        Self {
            client,
            table_name,
            partition_key,
            value_key,
        }
    }
}

#[async_trait]
impl KVStore for DynamoDBKVStore {
    async fn put<K, V>(&mut self, key: K, value: V) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Send,
        V: Serialize + Send,
    {
        let key = AttributeValue::S(key.as_str());
        let value = AttributeValue::S(serde_json::to_string(&value)?);
        let res = self
            .client
            .put_item()
            .table_name(&self.table_name)
            .item(&self.partition_key, key)
            .item(&self.value_key, value)
            .send();
        match res.await {
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
        let res = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key(&self.partition_key, AttributeValue::S(key));
        match res.send().await {
            Err(e) => Err(Box::new(e)),
            Ok(item) => match item.item() {
                None => return Ok(None),
                Some(item) => match item.get(&self.value_key) {
                    None => {
                        return Err(
                            format!("unexpected missing value field: {}", &self.value_key).into(),
                        )
                    }
                    Some(value) => match value.as_s() {
                        Ok(value) => match serde_json::from_str(value) {
                            Ok(value) => Ok(Some(value)),
                            Err(e) => Err(Box::new(e)),
                        },
                        Err(e) => {
                            return Err(
                                format!("error unwrapping value: {}", e.as_s().unwrap()).into()
                            )
                        }
                    },
                },
            },
        }
    }

    async fn exists<K>(&self, key: &K) -> Result<bool, Box<dyn Error>>
    where
        K: KeyString + Sync,
    {
        let key = key.as_str();
        let res = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .key(&self.partition_key, AttributeValue::S(key));
        match res.send().await {
            Err(e) => Err(Box::new(e)),
            Ok(item) => match item.item() {
                None => Ok(false),
                Some(_) => Ok(true),
            },
        }
    }

    async fn delete<K>(&mut self, key: &K) -> Result<(), Box<dyn Error>>
    where
        K: KeyString + Sync,
    {
        let key = key.as_str();
        let res = self
            .client
            .delete_item()
            .table_name(&self.table_name)
            .key(&self.partition_key, AttributeValue::S(key));
        match res.send().await {
            Err(e) => Err(Box::new(e)),
            Ok(_) => Ok(()),
        }
    }
}
