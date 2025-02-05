#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PruneRequest {
    #[prost(string, repeated, tag = "1")]
    pub filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "2")]
    pub all: bool,
    #[prost(int64, tag = "3")]
    pub keep_duration: i64,
    #[prost(int64, tag = "4")]
    pub keep_bytes: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskUsageRequest {
    #[prost(string, repeated, tag = "1")]
    pub filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskUsageResponse {
    #[prost(message, repeated, tag = "1")]
    pub record: ::prost::alloc::vec::Vec<UsageRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsageRecord {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub mutable: bool,
    #[prost(bool, tag = "3")]
    pub in_use: bool,
    #[prost(int64, tag = "4")]
    pub size: i64,
    #[deprecated]
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub created_at: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(message, optional, tag = "7")]
    pub last_used_at: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(int64, tag = "8")]
    pub usage_count: i64,
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub record_type: ::prost::alloc::string::String,
    #[prost(bool, tag = "11")]
    pub shared: bool,
    #[prost(string, repeated, tag = "12")]
    pub parents: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolveRequest {
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub definition: ::core::option::Option<super::super::super::pb::Definition>,
    #[prost(string, tag = "3")]
    pub exporter: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "4")]
    pub exporter_attrs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(string, tag = "5")]
    pub session: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub frontend: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "7")]
    pub frontend_attrs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    #[prost(message, optional, tag = "8")]
    pub cache: ::core::option::Option<CacheOptions>,
    #[prost(string, repeated, tag = "9")]
    pub entitlements: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "10")]
    pub frontend_inputs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::pb::Definition,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheOptions {
    /// ExportRefDeprecated is deprecated in favor or the new Exports since BuildKit v0.4.0.
    /// When ExportRefDeprecated is set, the solver appends
    /// {.Type = "registry", .Attrs = ExportAttrs.add("ref", ExportRef)}
    /// to Exports for compatibility. (planned to be removed)
    #[prost(string, tag = "1")]
    pub export_ref_deprecated: ::prost::alloc::string::String,
    /// ImportRefsDeprecated is deprecated in favor or the new Imports since BuildKit v0.4.0.
    /// When ImportRefsDeprecated is set, the solver appends
    /// {.Type = "registry", .Attrs = {"ref": importRef}}
    /// for each of the ImportRefs entry to Imports for compatibility. (planned to be removed)
    #[prost(string, repeated, tag = "2")]
    pub import_refs_deprecated: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ExportAttrsDeprecated is deprecated since BuildKit v0.4.0.
    /// See the description of ExportRefDeprecated.
    #[prost(map = "string, string", tag = "3")]
    pub export_attrs_deprecated: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Exports was introduced in BuildKit v0.4.0.
    #[prost(message, repeated, tag = "4")]
    pub exports: ::prost::alloc::vec::Vec<CacheOptionsEntry>,
    /// Imports was introduced in BuildKit v0.4.0.
    #[prost(message, repeated, tag = "5")]
    pub imports: ::prost::alloc::vec::Vec<CacheOptionsEntry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CacheOptionsEntry {
    /// Type is like "registry" or "local"
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Attrs are like mode=(min,max), ref=example.com:5000/foo/bar .
    /// See cache importer/exporter implementations' documentation.
    #[prost(map = "string, string", tag = "2")]
    pub attrs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SolveResponse {
    #[prost(map = "string, string", tag = "1")]
    pub exporter_response: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[prost(string, tag = "1")]
    pub r#ref: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(message, repeated, tag = "1")]
    pub vertexes: ::prost::alloc::vec::Vec<Vertex>,
    #[prost(message, repeated, tag = "2")]
    pub statuses: ::prost::alloc::vec::Vec<VertexStatus>,
    #[prost(message, repeated, tag = "3")]
    pub logs: ::prost::alloc::vec::Vec<VertexLog>,
    #[prost(message, repeated, tag = "4")]
    pub warnings: ::prost::alloc::vec::Vec<VertexWarning>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vertex {
    #[prost(string, tag = "1")]
    pub digest: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub inputs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub cached: bool,
    #[prost(message, optional, tag = "5")]
    pub started: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(message, optional, tag = "6")]
    pub completed: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    /// typed errors?
    #[prost(string, tag = "7")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub progress_group: ::core::option::Option<super::super::super::pb::ProgressGroup>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexStatus {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vertex: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub current: i64,
    #[prost(int64, tag = "5")]
    pub total: i64,
    #[prost(message, optional, tag = "6")]
    pub timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(message, optional, tag = "7")]
    pub started: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(message, optional, tag = "8")]
    pub completed: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexLog {
    #[prost(string, tag = "1")]
    pub vertex: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub timestamp: ::core::option::Option<
        super::super::super::google::protobuf::Timestamp,
    >,
    #[prost(int64, tag = "3")]
    pub stream: i64,
    #[prost(bytes = "vec", tag = "4")]
    pub msg: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VertexWarning {
    #[prost(string, tag = "1")]
    pub vertex: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub level: i64,
    #[prost(bytes = "vec", tag = "3")]
    pub short: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub detail: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, tag = "5")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub info: ::core::option::Option<super::super::super::pb::SourceInfo>,
    #[prost(message, repeated, tag = "7")]
    pub ranges: ::prost::alloc::vec::Vec<super::super::super::pb::Range>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytesMessage {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkersRequest {
    /// containerd style
    #[prost(string, repeated, tag = "1")]
    pub filter: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkersResponse {
    #[prost(message, repeated, tag = "1")]
    pub record: ::prost::alloc::vec::Vec<types::WorkerRecord>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoResponse {
    #[prost(message, optional, tag = "1")]
    pub buildkit_version: ::core::option::Option<types::BuildkitVersion>,
}
/// Generated server implementations.
pub mod control_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ControlServer.
    #[async_trait]
    pub trait Control: Send + Sync + 'static {
        async fn disk_usage(
            &self,
            request: tonic::Request<super::DiskUsageRequest>,
        ) -> Result<tonic::Response<super::DiskUsageResponse>, tonic::Status>;
        ///Server streaming response type for the Prune method.
        type PruneStream: futures_core::Stream<
                Item = Result<super::UsageRecord, tonic::Status>,
            >
            + Send
            + 'static;
        async fn prune(
            &self,
            request: tonic::Request<super::PruneRequest>,
        ) -> Result<tonic::Response<Self::PruneStream>, tonic::Status>;
        async fn solve(
            &self,
            request: tonic::Request<super::SolveRequest>,
        ) -> Result<tonic::Response<super::SolveResponse>, tonic::Status>;
        ///Server streaming response type for the Status method.
        type StatusStream: futures_core::Stream<
                Item = Result<super::StatusResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> Result<tonic::Response<Self::StatusStream>, tonic::Status>;
        ///Server streaming response type for the Session method.
        type SessionStream: futures_core::Stream<
                Item = Result<super::BytesMessage, tonic::Status>,
            >
            + Send
            + 'static;
        async fn session(
            &self,
            request: tonic::Request<tonic::Streaming<super::BytesMessage>>,
        ) -> Result<tonic::Response<Self::SessionStream>, tonic::Status>;
        async fn list_workers(
            &self,
            request: tonic::Request<super::ListWorkersRequest>,
        ) -> Result<tonic::Response<super::ListWorkersResponse>, tonic::Status>;
        async fn info(
            &self,
            request: tonic::Request<super::InfoRequest>,
        ) -> Result<tonic::Response<super::InfoResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ControlServer<T: Control> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Control> ControlServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ControlServer<T>
    where
        T: Control,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/moby.buildkit.v1.Control/DiskUsage" => {
                    #[allow(non_camel_case_types)]
                    struct DiskUsageSvc<T: Control>(pub Arc<T>);
                    impl<T: Control> tonic::server::UnaryService<super::DiskUsageRequest>
                    for DiskUsageSvc<T> {
                        type Response = super::DiskUsageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiskUsageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).disk_usage(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DiskUsageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/moby.buildkit.v1.Control/Prune" => {
                    #[allow(non_camel_case_types)]
                    struct PruneSvc<T: Control>(pub Arc<T>);
                    impl<
                        T: Control,
                    > tonic::server::ServerStreamingService<super::PruneRequest>
                    for PruneSvc<T> {
                        type Response = super::UsageRecord;
                        type ResponseStream = T::PruneStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PruneRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).prune(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PruneSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/moby.buildkit.v1.Control/Solve" => {
                    #[allow(non_camel_case_types)]
                    struct SolveSvc<T: Control>(pub Arc<T>);
                    impl<T: Control> tonic::server::UnaryService<super::SolveRequest>
                    for SolveSvc<T> {
                        type Response = super::SolveResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SolveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).solve(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SolveSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/moby.buildkit.v1.Control/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: Control>(pub Arc<T>);
                    impl<
                        T: Control,
                    > tonic::server::ServerStreamingService<super::StatusRequest>
                    for StatusSvc<T> {
                        type Response = super::StatusResponse;
                        type ResponseStream = T::StatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/moby.buildkit.v1.Control/Session" => {
                    #[allow(non_camel_case_types)]
                    struct SessionSvc<T: Control>(pub Arc<T>);
                    impl<T: Control> tonic::server::StreamingService<super::BytesMessage>
                    for SessionSvc<T> {
                        type Response = super::BytesMessage;
                        type ResponseStream = T::SessionStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::BytesMessage>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/moby.buildkit.v1.Control/ListWorkers" => {
                    #[allow(non_camel_case_types)]
                    struct ListWorkersSvc<T: Control>(pub Arc<T>);
                    impl<
                        T: Control,
                    > tonic::server::UnaryService<super::ListWorkersRequest>
                    for ListWorkersSvc<T> {
                        type Response = super::ListWorkersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListWorkersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_workers(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListWorkersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/moby.buildkit.v1.Control/Info" => {
                    #[allow(non_camel_case_types)]
                    struct InfoSvc<T: Control>(pub Arc<T>);
                    impl<T: Control> tonic::server::UnaryService<super::InfoRequest>
                    for InfoSvc<T> {
                        type Response = super::InfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::InfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = InfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Control> Clone for ControlServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Control> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Control> tonic::server::NamedService for ControlServer<T> {
        const NAME: &'static str = "moby.buildkit.v1.Control";
    }
}
