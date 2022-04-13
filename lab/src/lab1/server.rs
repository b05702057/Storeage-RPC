use tonic::Response;
use tribbler::{
    self,
    rpc,
    storage::{KeyValue, List, Pattern, Storage}, // to implement the rpcs
};

// declare a new struct and add fileds to it
pub struct StorageServer {
    pub storage: Box<dyn Storage>,
}

#[async_trait::async_trait]
impl rpc::trib_storage_server::TribStorage for StorageServer {
    async fn get(
        &self,
        request: tonic::Request<rpc::Key>,
    ) -> Result<tonic::Response<rpc::Value>, tonic::Status> {
        let k = request.into_inner(); // get the message in the request
        let output = self.storage.get(k.key.as_str()).await;
        match output {
            Ok(Some(t)) => Ok(Response::new(rpc::Value { value: t })),
            Ok(None) => Ok(Response::new(rpc::Value {
                value: "".to_string(),
            })),
            Err(e) => Err(tonic::Status::unknown("fail to get")),
        }
    }

    async fn set(
        &self,
        request: tonic::Request<rpc::KeyValue>,
    ) -> Result<tonic::Response<rpc::Bool>, tonic::Status> {
        let kv = request.into_inner();
        let output = self
            .storage
            .set(&KeyValue {
                key: kv.key,
                value: kv.value,
            })
            .await;
        match output {
            Ok(t) => Ok(Response::new(rpc::Bool { value: t })),
            Err(e) => Err(tonic::Status::unknown("fail to set")),
        }
    }

    async fn keys(
        &self,
        request: tonic::Request<rpc::Pattern>,
    ) -> Result<tonic::Response<rpc::StringList>, tonic::Status> {
        let p = request.into_inner();
        let output = self
            .storage
            .keys(&Pattern {
                prefix: p.prefix,
                suffix: p.suffix,
            })
            .await;
        match output {
            Ok(List(t)) => Ok(Response::new(rpc::StringList { list: t })),
            Err(e) => Err(tonic::Status::unknown("fail keys")),
        }
    }

    async fn list_get(
        &self,
        request: tonic::Request<rpc::Key>,
    ) -> Result<tonic::Response<rpc::StringList>, tonic::Status> {
        let k = request.into_inner();
        let output = self.storage.list_get(k.key.as_str()).await;
        match output {
            Ok(List(t)) => Ok(Response::new(rpc::StringList { list: t })),
            Err(e) => Err(tonic::Status::unknown("fail keys")),
        }
    }

    async fn list_append(
        &self,
        request: tonic::Request<rpc::KeyValue>,
    ) -> Result<tonic::Response<rpc::Bool>, tonic::Status> {
        let kv = request.into_inner();
        let output = self
            .storage
            .list_append(&KeyValue {
                key: kv.key,
                value: kv.value,
            })
            .await;
        match output {
            Ok(t) => Ok(Response::new(rpc::Bool { value: t })),
            Err(e) => Err(tonic::Status::unknown("fail list_append")),
        }
    }

    async fn list_remove(
        &self,
        request: tonic::Request<rpc::KeyValue>,
    ) -> Result<tonic::Response<rpc::ListRemoveResponse>, tonic::Status> {
        let kv = request.into_inner();
        let output = self
            .storage
            .list_remove(&KeyValue {
                key: kv.key,
                value: kv.value,
            })
            .await;
        match output {
            Ok(t) => Ok(Response::new(rpc::ListRemoveResponse { removed: t })),
            Err(e) => Err(tonic::Status::unknown("fail list_remove")),
        }
    }

    async fn list_keys(
        &self,
        request: tonic::Request<rpc::Pattern>,
    ) -> Result<tonic::Response<rpc::StringList>, tonic::Status> {
        let p = request.into_inner();
        let output = self
            .storage
            .list_keys(&Pattern {
                prefix: p.prefix,
                suffix: p.suffix,
            })
            .await;
        match output {
            Ok(List(t)) => Ok(Response::new(rpc::StringList { list: t })),
            Err(e) => Err(tonic::Status::unknown("fail list_keys")),
        }
    }

    async fn clock(
        &self,
        request: tonic::Request<rpc::Clock>,
    ) -> Result<tonic::Response<rpc::Clock>, tonic::Status> {
        let t = request.into_inner();
        let output = self.storage.clock(t.timestamp).await;
        match output {
            Ok(t) => Ok(Response::new(rpc::Clock { timestamp: t })),
            Err(e) => Err(tonic::Status::unknown("fail clock")),
        }
    }
}
