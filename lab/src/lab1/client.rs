// use path::item
use async_trait::async_trait;
use tribbler::{
    self,
    err::TribResult,
    rpc,
    rpc::trib_storage_client::TribStorageClient,
    storage::{KeyList, KeyString, KeyValue, List, Pattern, Storage}, // to implement the RPCs
};

// declare a new struct and add fileds to it (addr)
pub struct StorageClient {
    pub addr: String, // note that str and String are distinct types => let _ = StorageClient { addr: addr.to_string() };
}

// assume that each call on the same key is an atomic transaction
#[async_trait] // VERY IMPORTANT !! => The async features are new, and the compiler doesn't support them in trait definition, so we need this line.
impl KeyString for StorageClient {
    // add method implementations to match the tribbler::storage::Storage trait
    async fn get(&self, key: &str) -> TribResult<Option<String>> {
        // acceptable but not efficient since we open a connection for each RPC call
        let mut client = TribStorageClient::connect(self.addr.clone()).await?; // wait until we need to perform the first RPC function call
        let r = client
            .get(rpc::Key {
                key: key.to_string(),
            })
            .await?; // "?" replaces the common syntax for error handling
                     // https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/reference/expressions/operator-expr.html

        // Rust methods are named into_something when they consume themselves, avoiding clones as much as possible.
        match r.into_inner().value.as_str() {
            // consumes self and return an inner, wrapped object (unwrap Response)
            "" => Ok(None),                       // as_str() for ""
            value => Ok(Some(value.to_string())), // match any value
        }
    }

    // This kv passed by the user should be the KeyValue struct of the storage because the user should use the storage as if he has it.
    async fn set(&self, kv: &KeyValue) -> TribResult<bool> {
        // modify key value
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .set(rpc::KeyValue {
                key: kv.key.clone(),
                value: kv.value.clone(),
            })
            .await?;
        match r.into_inner().value {
            value => Ok(value),
        }
    }

    async fn keys(&self, p: &Pattern) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .keys(rpc::Pattern {
                prefix: p.prefix.clone(),
                suffix: p.suffix.clone(),
            })
            .await?;
        match r.into_inner().list {
            list => Ok(List(list)),
        }
    }
}

#[async_trait]
impl KeyList for StorageClient {
    async fn list_get(&self, key: &str) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_get(rpc::Key {
                key: key.to_string(),
            })
            .await?;
        match r.into_inner().list {
            list => Ok(List(list)),
        }
    }

    async fn list_append(&self, kv: &KeyValue) -> TribResult<bool> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_append(rpc::KeyValue {
                key: kv.key.clone(),
                value: kv.value.clone(),
            })
            .await?;
        match r.into_inner().value {
            value => Ok(value),
        }
    }

    async fn list_remove(&self, kv: &KeyValue) -> TribResult<u32> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_remove(rpc::KeyValue {
                key: kv.key.clone(),
                value: kv.value.clone(),
            })
            .await?;
        match r.into_inner().removed {
            removed => Ok(removed),
        }
    }

    async fn list_keys(&self, p: &Pattern) -> TribResult<List> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .list_keys(rpc::Pattern {
                prefix: p.prefix.clone(),
                suffix: p.suffix.clone(),
            })
            .await?;
        match r.into_inner().list {
            list => Ok(List(list)),
        }
    }
}

#[async_trait]
impl Storage for StorageClient {
    async fn clock(&self, at_least: u64) -> TribResult<u64> {
        let mut client = TribStorageClient::connect(self.addr.clone()).await?;
        let r = client
            .clock(rpc::Clock {
                timestamp: at_least,
            })
            .await?;
        match r.into_inner().timestamp {
            timestamp => Ok(timestamp),
        }
    }
}
