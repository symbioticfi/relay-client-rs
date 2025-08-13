# Relay Client (Rust)

Rust gRPC client for the Symbiotic Relay. Code is generated with buf using prost + tonic. Generated modules live under `crate::generated`.

# Usage

```rust
use relay_client_rs::generated::api::proto::v1::symbiotic_api_service_client::SymbioticApiServiceClient;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Endpoint::from_shared("http://localhost:8080")?.connect().await?;
    let mut client = SymbioticApiServiceClient::new(channel);

    // Use the client; e.g. see examples for requests and streaming usage
    Ok(())
}
```

For more usage (requests, streaming, helpers), see `examples/basic_usage.rs` in the [examples](./examples) directory.

# Development

Run scripts/update-proto.sh to fetch upstream proto and regenerate; run cargo build to compile.
