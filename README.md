# Generic Key Value store for rocksdb and dynomodb

## Goal
- Generic key value storage for custom defined key and value type

## Usage
- full example: `main.rs`

- AWS dynomodb
```toml
[dependencies]
tokio = { version = "1.28", features = ["full"] }
kvstore = { git = "https://github.com/speed2exe/kvstore", branch = "main"}
serde_json = "1.0"
serde = { version = "1.0", features = ["derive"] }
# rocksdb = "0.21"
aws-config = "0.55"
aws-sdk-dynamodb = "0.28"
```

```rust
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;
use kvstore::{dynamodb_kv::DynamoDBKVStore, KVStore};
use std::error::Error;

use serde::{Deserialize, Serialize};

struct CustomKey {
    // ...
}

impl kvstore::KeyString for CustomKey {
    fn as_str(&self) -> String {
        // unique string for this key
    }
}

struct CustomValue {
    // ...
}

#[tokio::main]
async fn main() {
    // ~/.aws/credentials
    // [default]
    // aws_access_key_id = <your access key>
    // aws_secret_access_key = <your secret key>
    let aws_dynomodb_client = {
        let region_provider = RegionProviderChain::default_provider().or_else("ap-southeast-1");
        let config = aws_config::from_env().region(region_provider).load().await;
        Client::new(&config)
    };
    let mut dynamodb_store = DynamoDBKVStore::new(
        aws_dynomodb_client,
        "your_table".to_string(),
        "your_partition".to_string(),
        "value_field".to_string(), // can be fixed defined value
    );

    let key = CustomKey {
        // ...
    };
    let value = CustomValue {
        // ...
    };

    // put data
    let put_res: Result<(), Box<dyn Error>> = kvstore.put(key, value).await;

    // get data
    let get_res: Result<Option<CustomValue>, Box<dyn Error>> = kvstore.get(&key).await;

    // delete data
    let del_res: Result<(), Box<dyn Error>> = kvstore.delete(&key2).await;

    // check exist
    let put_res: Result<bool, Box<dyn Error>> = kvstore.put(key, value).await;
}
```

## Methodology
- Uses a unique string to represent key
- Uses JSON string to represent underlying values in various storage system

## Requirements
- custom key types need to implement `KeyString` trait
```rust
struct CustomKey {
    id: i32,
    name: String,
}

impl kvstore::KeyString for CustomKey {
    fn as_str(&self) -> String {
        format!("{}-{}", self.id, self.name)
    }
}
```

- custom values need to implement `serde::{Deserialize, Serialize}` trait

## Possible Future improvements
- use `#![feature(async_fn_in_trait)]`
- RWLock on `KeyString`
