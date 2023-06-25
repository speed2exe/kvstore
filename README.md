# Generic Key Value store for rocksdb and dynomodb

## Goal
- Generic key value storage for custom defined key and value type

## Usage
- see `main.rs`

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
