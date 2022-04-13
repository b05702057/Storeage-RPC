use crate::lab1::client::StorageClient;
use crate::lab1::server::StorageServer;
use std::boxed::Box;
use std::net::ToSocketAddrs;
use tonic::transport::Server;
use tribbler::{
    self,
    err::TribResult,
    rpc::trib_storage_server::TribStorageServer,
    {config::BackConfig, storage::Storage},
};

/// an async function which blocks indefinitely (unlimited time) until interrupted serving on the host and port specified in the [BackConfig] parameter.
pub async fn serve_back(config: BackConfig) -> TribResult<()> {
    // creates an instance of a back-end server based on configuration
    let storage_server = StorageServer {
        storage: config.storage,
    };

    // The server is ready if it reaches this line.
    let _ = match config.ready {
        Some(unwrapped_ready) => unwrapped_ready.send(true),
        None => Ok(()),
    };
    match config.addr.to_socket_addrs() {
        Ok(iterator) => match iterator.last() {
            Some(socket_addr) => {
                match config.shutdown {
                    Some(mut s) => {
                        Server::builder()
                            .add_service(TribStorageServer::new(storage_server))
                            .serve_with_shutdown(socket_addr, async {
                                s.recv().await;
                            }) // block until there is an error, or a shutdown message is received
                            .await?
                    }
                    None => {
                        Server::builder()
                            .add_service(TribStorageServer::new(storage_server))
                            .serve(socket_addr)
                            .await?
                    }
                }
            }
            None => return Ok(()),
        },
        Err(e) => return Err(Box::new(e)),
    };
    Ok(())
}
// post 44, 60, and 63

/// This function should create a new client which implements the [Storage] trait.
/// It should communicate with the backend that is started in the [serve_back] function.
pub async fn new_client(addr: &str) -> TribResult<Box<dyn Storage>> {
    Ok(Box::new(StorageClient {
        // wrap a new client obeject with Ok(Box::new()) for the type constraint
        addr: addr.to_string(), // &str and String are distinct types in Rust.
    }))
}
