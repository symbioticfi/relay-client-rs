//! Rust gRPC client library for Symbiotic Relay
//!
//! This library provides a client interface to communicate with Symbiotic Relay servers
//! using gRPC. It includes auto-generated client code from protocol buffer definitions.
//!
//! # Usage
//!
//! ```rust,no_run
//! use relay_client::generated::api::proto::v1::symbiotic_api_service_client::SymbioticApiServiceClient;
//! use tonic::transport::Endpoint;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let endpoint = Endpoint::from_shared("http://localhost:8080")?;
//!     let channel = endpoint.connect().await?;
//!     let client = SymbioticApiServiceClient::new(channel);
//!     // Use the client...
//!     Ok(())
//! }
//! ```

pub mod generated;

// Re-export commonly used types for convenience
pub use generated::api::proto::v1::*;
