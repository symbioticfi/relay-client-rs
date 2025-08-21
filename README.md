# Symbiotic Relay Client (Rust)

Rust gRPC client for the Symbiotic Relay. Code is generated with buf using prost + tonic. Generated modules live under `crate::generated`.

# Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
symbiotic-relay-client = "0.2.0"
```

## Using Development/Nightly Versions

If you want to use the latest development version instead of a stable release, you can use a git dependency:

```toml
[dependencies]
symbiotic-relay-client = { git = "https://github.com/symbioticfi/relay-client-rs", rev = "9f35b8e" }
```

Replace `9f35b8e` with the specific commit hash you want to use. You can also use:
- `branch = "main"` for the latest main branch
- `tag = "v0.2.0"` for a specific release tag

# Usage

```rust
use symbiotic_relay_client::generated::api::proto::v1::symbiotic_api_service_client::SymbioticApiServiceClient;
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
