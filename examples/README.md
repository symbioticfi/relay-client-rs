# Symbiotic Relay Client Examples

This directory contains examples demonstrating how to use the `relay-client-rs` library to interact with Symbiotic Relay servers.

## Basic Usage Example

The `basic_usage.rs` example shows how to:

1. Connect to a Symbiotic Relay server
2. Get current epoch information
3. Sign messages 
4. Retrieve aggregation proofs and signatures
5. Get validator set information
6. Use streaming responses for real-time updates

## Running the Example

```bash
cd examples
cargo run --bin basic_usage
```

By default, the example will try to connect to `http://localhost:8080`. You can specify a different server URL by setting the `RELAY_SERVER_URL` environment variable:

```bash
RELAY_SERVER_URL=http://my-relay-server:8080 cargo run --bin basic_usage
```

NOTE: for the signature/proof generation to work you need to run the script for all active relay servers to get the majority consensus to generate proof.

## Creating a Client

To use the relay client in your own project:

1. Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
relay-client-rs = { path = "../path/to/relay-client-rs" }
tonic = "0.12"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

2. Create a client in your code:
```rust
use relay_client::gen::api::proto::v1::symbiotic_api_service_client::SymbioticApiServiceClient;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create transport channel
    let endpoint = Endpoint::from_shared("http://localhost:8080")?;
    let channel = endpoint.connect().await?;
    
    // Create the gRPC client
    let mut client = SymbioticApiServiceClient::new(channel);
    
    // Use the client for API calls
    let response = client.get_current_epoch(
        tonic::Request::new(relay_client::GetCurrentEpochRequest {})
    ).await?;
    
    println!("Current epoch: {}", response.into_inner().epoch);
    
    Ok(())
}
```

## Configuration Options

You can configure the transport connection with various options:

```rust
use tonic::transport::Endpoint;
use std::time::Duration;

let endpoint = Endpoint::from_shared("http://localhost:8080")?
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(10))
    .tcp_keepalive(Some(Duration::from_secs(60)));

let channel = endpoint.connect().await?;
```

For more advanced configuration options, see the [tonic documentation](https://docs.rs/tonic/).