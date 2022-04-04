#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyValue {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pattern {
    #[prost(string, tag = "1")]
    pub prefix: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub suffix: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bool {
    #[prost(bool, tag = "1")]
    pub value: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Key {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringList {
    #[prost(string, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Clock {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRemoveResponse {
    #[prost(uint32, tag = "1")]
    pub removed: u32,
}
#[doc = r" Generated client implementations."]
pub mod trib_storage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct TribStorageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TribStorageClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TribStorageClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> TribStorageClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            TribStorageClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<tonic::Response<super::Value>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValue>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/set");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn keys(
            &mut self,
            request: impl tonic::IntoRequest<super::Pattern>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/keys");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_get(
            &mut self,
            request: impl tonic::IntoRequest<super::Key>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/listGet");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_append(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValue>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/listAppend");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_remove(
            &mut self,
            request: impl tonic::IntoRequest<super::KeyValue>,
        ) -> Result<tonic::Response<super::ListRemoveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/listRemove");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::Pattern>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/listKeys");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn clock(
            &mut self,
            request: impl tonic::IntoRequest<super::Clock>,
        ) -> Result<tonic::Response<super::Clock>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/rpc.TribStorage/clock");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod trib_storage_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TribStorageServer."]
    #[async_trait]
    pub trait TribStorage: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::Key>,
        ) -> Result<tonic::Response<super::Value>, tonic::Status>;
        async fn set(
            &self,
            request: tonic::Request<super::KeyValue>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status>;
        async fn keys(
            &self,
            request: tonic::Request<super::Pattern>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status>;
        async fn list_get(
            &self,
            request: tonic::Request<super::Key>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status>;
        async fn list_append(
            &self,
            request: tonic::Request<super::KeyValue>,
        ) -> Result<tonic::Response<super::Bool>, tonic::Status>;
        async fn list_remove(
            &self,
            request: tonic::Request<super::KeyValue>,
        ) -> Result<tonic::Response<super::ListRemoveResponse>, tonic::Status>;
        async fn list_keys(
            &self,
            request: tonic::Request<super::Pattern>,
        ) -> Result<tonic::Response<super::StringList>, tonic::Status>;
        async fn clock(
            &self,
            request: tonic::Request<super::Clock>,
        ) -> Result<tonic::Response<super::Clock>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct TribStorageServer<T: TribStorage> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: TribStorage> TribStorageServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for TribStorageServer<T>
    where
        T: TribStorage,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/rpc.TribStorage/get" => {
                    #[allow(non_camel_case_types)]
                    struct getSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::Key> for getSvc<T> {
                        type Response = super::Value;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Key>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/set" => {
                    #[allow(non_camel_case_types)]
                    struct setSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::KeyValue> for setSvc<T> {
                        type Response = super::Bool;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyValue>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = setSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/keys" => {
                    #[allow(non_camel_case_types)]
                    struct keysSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::Pattern> for keysSvc<T> {
                        type Response = super::StringList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Pattern>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = keysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/listGet" => {
                    #[allow(non_camel_case_types)]
                    struct listGetSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::Key> for listGetSvc<T> {
                        type Response = super::StringList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Key>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listGetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/listAppend" => {
                    #[allow(non_camel_case_types)]
                    struct listAppendSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::KeyValue> for listAppendSvc<T> {
                        type Response = super::Bool;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyValue>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_append(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listAppendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/listRemove" => {
                    #[allow(non_camel_case_types)]
                    struct listRemoveSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::KeyValue> for listRemoveSvc<T> {
                        type Response = super::ListRemoveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::KeyValue>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_remove(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listRemoveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/listKeys" => {
                    #[allow(non_camel_case_types)]
                    struct listKeysSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::Pattern> for listKeysSvc<T> {
                        type Response = super::StringList;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Pattern>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = listKeysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/rpc.TribStorage/clock" => {
                    #[allow(non_camel_case_types)]
                    struct clockSvc<T: TribStorage>(pub Arc<T>);
                    impl<T: TribStorage> tonic::server::UnaryService<super::Clock> for clockSvc<T> {
                        type Response = super::Clock;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Clock>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).clock(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = clockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TribStorage> Clone for TribStorageServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: TribStorage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TribStorage> tonic::transport::NamedService for TribStorageServer<T> {
        const NAME: &'static str = "rpc.TribStorage";
    }
}
