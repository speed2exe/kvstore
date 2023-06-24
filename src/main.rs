// import from lib.rs
use core::hash::Hash;
use std::{cmp::Eq, collections::HashMap};

use kvstore::{mem_kv::MemKVStore, KVStore};

#[derive(Hash, Eq, PartialEq, Debug)]
struct CustomKey {
    id: i32,
    name: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct CustomValue {
    id: i32,
    name: String,

    settings: HashMap<String, String>,
}

async fn custom_kvstore_operations(kvstore: &mut impl KVStore<CustomKey, CustomValue>) {
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
    let value2 = kvstore.get(&key2).await.unwrap();
    println!("value2: {:?}", value2);

    let is_exists = kvstore.exists(&key2).await.unwrap();
    println!("is_exists: {:?}", is_exists);

    // // should not exist after deletion
    kvstore.delete(&key2).await.unwrap();
    let value3 = kvstore.get(&key2).await.unwrap();
    println!("value3: {:?}", value3);

    let is_exists = kvstore.exists(&key2).await.unwrap();
    println!("is_exists: {:?}", is_exists);
}

#[tokio::main]
async fn main() {
    // concrete type
    let mut store = MemKVStore::<CustomKey, CustomValue>::new();

    // pass in as trait
    custom_kvstore_operations(&mut store).await;

    println!("store: {:?}", store);
}
