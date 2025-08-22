//! Basic usage example for the Symbiotic Relay Rust client.
//!
//! This example demonstrates how to:
//! 1. Connect to a Symbiotic Relay server
//! 2. Get the current epoch
//! 3. Sign a message
//! 4. Retrieve aggregation proofs
//! 5. Get validator set information

use std::env;
use symbiotic_relay_client::generated::api::proto::v1::{
    GetAggregationProofRequest, GetCurrentEpochRequest, GetSignaturesRequest,
    GetSuggestedEpochRequest, GetValidatorSetRequest, SignMessageRequest, SignMessageWaitRequest,
    SigningStatus, symbiotic_api_service_client::SymbioticApiServiceClient,
};
use tokio_stream::StreamExt;
use tonic::transport::Channel;

/// Simple wrapper around the generated gRPC client.
pub struct RelayClient {
    client: SymbioticApiServiceClient<Channel>,
}

impl RelayClient {
    /// Create a new client connected to the specified server URL.
    pub async fn new(server_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let endpoint = tonic::transport::Endpoint::from_shared(server_url.to_string())?;
        let channel = endpoint.connect().await?;
        let client = SymbioticApiServiceClient::new(channel);

        println!("Connected to Symbiotic Relay at {}", server_url);

        Ok(Self { client })
    }

    /// Get the current epoch information.
    pub async fn get_current_epoch(
        &mut self,
    ) -> Result<
        tonic::Response<symbiotic_relay_client::generated::api::proto::v1::GetCurrentEpochResponse>,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetCurrentEpochRequest {});
        self.client.get_current_epoch(request).await
    }

    /// Get the suggested epoch for signing.
    pub async fn get_suggested_epoch(
        &mut self,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetSuggestedEpochResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetSuggestedEpochRequest {});
        self.client.get_suggested_epoch(request).await
    }

    /// Sign a message using the specified key tag.
    pub async fn sign_message(
        &mut self,
        key_tag: u32,
        message: Vec<u8>,
        required_epoch: Option<u64>,
    ) -> Result<
        tonic::Response<symbiotic_relay_client::generated::api::proto::v1::SignMessageResponse>,
        tonic::Status,
    > {
        let request = tonic::Request::new(SignMessageRequest {
            key_tag,
            message: message.into(),
            required_epoch,
        });
        self.client.sign_message(request).await
    }

    /// Get aggregation proof for a specific request.
    pub async fn get_aggregation_proof(
        &mut self,
        request_hash: String,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetAggregationProofResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetAggregationProofRequest { request_hash });
        self.client.get_aggregation_proof(request).await
    }

    /// Get individual signatures for a request.
    pub async fn get_signatures(
        &mut self,
        request_hash: String,
    ) -> Result<
        tonic::Response<symbiotic_relay_client::generated::api::proto::v1::GetSignaturesResponse>,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetSignaturesRequest { request_hash });
        self.client.get_signatures(request).await
    }

    /// Get validator set information.
    pub async fn get_validator_set(
        &mut self,
        epoch: Option<u64>,
    ) -> Result<
        tonic::Response<symbiotic_relay_client::generated::api::proto::v1::GetValidatorSetResponse>,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetValidatorSetRequest { epoch });
        self.client.get_validator_set(request).await
    }

    /// Sign a message and wait for aggregation via streaming response.
    pub async fn sign_message_and_wait(
        &mut self,
        key_tag: u32,
        message: Vec<u8>,
        required_epoch: Option<u64>,
    ) -> Result<
        tonic::Streaming<
            symbiotic_relay_client::generated::api::proto::v1::SignMessageWaitResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(SignMessageWaitRequest {
            key_tag,
            message: message.into(),
            required_epoch,
        });
        let response = self.client.sign_message_wait(request).await?;
        Ok(response.into_inner())
    }
}

/// Main example function demonstrating client usage.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let server_url =
        env::var("RELAY_SERVER_URL").unwrap_or_else(|_| "localhost:8080".to_string());
    let mut client = RelayClient::new(&server_url).await?;

    // Example 1: Get current epoch
    println!("=== Getting Current Epoch ===");
    let epoch_response = client.get_current_epoch().await?;
    let epoch_data = epoch_response.into_inner();
    println!("Current epoch: {}", epoch_data.epoch);
    if let Some(start_time) = epoch_data.start_time {
        println!("Start time: {:?}", start_time);
    }

    // Example 2: Get suggested epoch
    println!("\n=== Getting Suggested Epoch ===");
    let suggested_epoch = client.get_suggested_epoch().await?;
    let suggested_data = suggested_epoch.into_inner();
    println!("Suggested epoch: {}", suggested_data.epoch);

    // Example 3: Get validator set
    println!("\n=== Getting Validator Set ===");
    let validator_set = client.get_validator_set(None).await?;
    let validator_data = validator_set.into_inner();
    println!("Validator set version: {}", validator_data.version);
    println!("Epoch: {}", validator_data.epoch);
    println!("Status: {:?}", validator_data.status());
    println!("Number of validators: {}", validator_data.validators.len());

    // Display some validator details
    if let Some(first_validator) = validator_data.validators.first() {
        println!("First validator operator: {}", first_validator.operator);
        println!(
            "First validator voting power: {}",
            first_validator.voting_power
        );
        println!("First validator is active: {}", first_validator.is_active);
        println!("First validator keys count: {}", first_validator.keys.len());
    }

    // Example 4: Sign a message
    println!("\n=== Signing a Message ===");
    let message_to_sign = "Hello, Symbiotic!".as_bytes().to_vec();
    let key_tag = 15;

    let sign_response = client.sign_message(key_tag, message_to_sign, None).await?;
    let sign_data = sign_response.into_inner();
    println!("Request hash: {}", sign_data.request_hash);
    println!("Epoch: {}", sign_data.epoch);

    // Example 5: Get aggregation proof (this might fail if signing is not complete)
    println!("\n=== Getting Aggregation Proof ===");
    match client
        .get_aggregation_proof(sign_data.request_hash.clone())
        .await
    {
        Ok(proof_response) => {
            let proof_data = proof_response.into_inner();
            if let Some(proof) = proof_data.aggregation_proof {
                println!("Verification type: {}", proof.verification_type);
                println!("Proof length: {} bytes", proof.proof.len());
            }
        }
        Err(e) => {
            println!("Could not get aggregation proof yet: {}", e.message());
        }
    }

    // Example 6: Get individual signatures
    println!("\n=== Getting Individual Signatures ===");
    match client.get_signatures(sign_data.request_hash.clone()).await {
        Ok(signatures_response) => {
            let signatures_data = signatures_response.into_inner();
            println!("Number of signatures: {}", signatures_data.signatures.len());

            for (index, signature) in signatures_data.signatures.iter().enumerate() {
                println!("Signature {}:", index + 1);
                println!("  - Signature length: {} bytes", signature.signature.len());
                println!(
                    "  - Public key length: {} bytes",
                    signature.public_key.len()
                );
                println!(
                    "  - Message hash length: {} bytes",
                    signature.message_hash.len()
                );
            }
        }
        Err(e) => {
            println!("Could not get signatures yet: {}", e.message());
        }
    }

    // Example 7: Sign and wait for completion (streaming)
    println!("\n=== Sign and Wait (Streaming) ===");
    let message_to_sign_stream = "Streaming example".as_bytes().to_vec();

    println!(
        "Starting streaming sign request...(ensure to run the script for all active relay servers)"
    );
    let mut stream = client
        .sign_message_and_wait(key_tag, message_to_sign_stream, None)
        .await?;

    while let Some(stream_response) = stream.next().await {
        match stream_response {
            Ok(response) => {
                let status =
                    SigningStatus::try_from(response.status).unwrap_or(SigningStatus::Unspecified);
                println!("Status: {:?}", status);
                println!("Request hash: {}", response.request_hash);

                match status {
                    SigningStatus::Pending => {
                        println!("Request created, waiting for signatures...");
                    }
                    SigningStatus::Completed => {
                        println!("Signing completed!");
                        if let Some(proof) = response.aggregation_proof {
                            println!("Proof length: {} bytes", proof.proof.len());
                            println!("Verification type: {}", proof.verification_type);
                        }
                        break; // Exit the streaming loop
                    }
                    SigningStatus::Failed => {
                        println!("Signing failed");
                        break;
                    }
                    SigningStatus::Timeout => {
                        println!("Signing timed out");
                        break;
                    }
                    _ => {
                        println!("Unknown status: {:?}", status);
                    }
                }

                // Add a small delay to make the output more readable
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
            Err(e) => {
                println!("Stream error: {}", e.message());
                break;
            }
        }
    }

    println!("\nExample completed");
    Ok(())
}
