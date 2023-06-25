use std::collections::HashMap;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

use kvstore::{
    dynamodb_kv::DynamoDBKVStore, mem_kv::MemKVStore, rocksdb_kv::RocksDBKVStore, KVStore,
};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct CustomKey {
    id: i32,
    name: String,
}

impl kvstore::KeyString for CustomKey {
    fn as_str(&self) -> String {
        format!("{}-{}", self.id, self.name)
    }
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct CustomValue {
    id: i32,
    name: String,

    settings: HashMap<String, String>,
}

async fn custom_kvstore_operations(kvstore: &mut impl KVStore) {
    let key = CustomKey {
        id: 1,
        name: "test".to_string(),
    };
    let value = CustomValue {
        id: 1,
        name: "test".to_string(),
        settings: HashMap::from([
            ("key1".to_string(), "value1".to_string()),
            ("key2".to_string(), "value2".to_string()),
            ("key3".to_string(), "value3".to_string()),
        ]),
    };

    // put data
    let put_res = kvstore.put(key, value).await;
    assert!(put_res.is_ok());

    // creating a new key with same id and name
    let key2 = CustomKey {
        name: "test".to_string(),
        id: 1,
    };
    let get_res = kvstore.get::<CustomKey, CustomValue>(&key2).await;
    // should get back the same value
    assert!(get_res.is_ok());
    let payload = get_res.unwrap().unwrap();
    println!("res_value from kvstore: {:?}", &payload);
    let key1value = payload.settings.get("key1").unwrap();
    assert_eq!(key1value, "value1");

    // should exist after put
    let is_exists = kvstore.exists(&key2).await.unwrap();
    assert!(is_exists);

    // delete
    let del_res = kvstore.delete(&key2).await;
    assert!(del_res.is_ok());

    // result is none after deletion
    let value3: Option<CustomValue> = kvstore.get(&key2).await.unwrap();
    assert!(value3.is_none());

    // should not exist after deletion
    let is_exists = kvstore.exists(&key2).await.unwrap();
    assert!(!is_exists);
}

#[tokio::main]
async fn main() {
    // basic memory store
    let mut mem_store = MemKVStore::new();
    custom_kvstore_operations(&mut mem_store).await;

    // rocksdb store
    let rocksdb = rocksdb::DB::open_default("rocksdb").unwrap();
    let mut rocksdb_store = RocksDBKVStore::new(rocksdb);
    custom_kvstore_operations(&mut rocksdb_store).await;

    // dynamodb store
    let aws_dynomodb_client = {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        Client::new(&config)
    };
    let mut dynamodb_store = DynamoDBKVStore::new(
        aws_dynomodb_client,
        "table1".to_string(),
        "part1".to_string(),
        "value".to_string(),
    );
    custom_kvstore_operations(&mut dynamodb_store).await;
}
