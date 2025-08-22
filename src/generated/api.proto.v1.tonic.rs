// @generated
/// Generated client implementations.
pub mod symbiotic_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SymbioticApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SymbioticApiServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SymbioticApiServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SymbioticApiServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SymbioticApiServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn sign_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SignMessageResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/SignMessage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("api.proto.v1.SymbioticAPIService", "SignMessage"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_aggregation_proof(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAggregationProofRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAggregationProofResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetAggregationProof",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetAggregationProof",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_current_epoch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCurrentEpochRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCurrentEpochResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetCurrentEpoch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetCurrentEpoch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_suggested_epoch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSuggestedEpochRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSuggestedEpochResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetSuggestedEpoch",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetSuggestedEpoch",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_signatures(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSignaturesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSignaturesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetSignatures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("api.proto.v1.SymbioticAPIService", "GetSignatures"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_signature_request(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSignatureRequestRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSignatureRequestResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetSignatureRequest",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetSignatureRequest",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_aggregation_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAggregationStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAggregationStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetAggregationStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetAggregationStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_validator_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorSetRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorSetResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetValidatorSet",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetValidatorSet",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_validator_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorByAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorByAddressResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetValidatorByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetValidatorByAddress",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_validator_set_header(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorSetHeaderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorSetHeaderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/GetValidatorSetHeader",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "GetValidatorSetHeader",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn sign_message_wait(
            &mut self,
            request: impl tonic::IntoRequest<super::SignMessageWaitRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::SignMessageWaitResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/api.proto.v1.SymbioticAPIService/SignMessageWait",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "api.proto.v1.SymbioticAPIService",
                        "SignMessageWait",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
