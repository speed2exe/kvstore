use std::collections::HashMap;

use kvstore::{mem_kv::MemKVStore, rocksdb_kv::RocksDBKVStore, KVStore};
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
    kvstore.put(key, value).await.unwrap();

    let key2 = CustomKey {
        id: 1,
        name: "test".to_string(),
    };
    let value2: Option<CustomValue> = kvstore.get(&key2).await.unwrap();
    println!("value2: {:?}", value2);

    let is_exists = kvstore.exists(&key2).await.unwrap();
    println!("is_exists: {:?}", is_exists);

    // should not exist after deletion
    kvstore.delete(&key2).await.unwrap();
    let value3: Option<CustomValue> = kvstore.get(&key2).await.unwrap();
    println!("value3: {:?}", value3);

    let is_exists = kvstore.exists(&key2).await.unwrap();
    println!("is_exists: {:?}", is_exists);
}

#[tokio::main]
async fn main() {
    // concrete type
    let mut mem_store = MemKVStore::new();
    custom_kvstore_operations(&mut mem_store).await;

    let mut rocksdb_store = RocksDBKVStore::new("rocksdb").unwrap();
    custom_kvstore_operations(&mut rocksdb_store).await;

    println!("store: {:?}", mem_store);
}
