//! Basic usage example for the Symbiotic Relay Rust client.
//!
//! This example demonstrates how to:
//! 1. Connect to a Symbiotic Relay server
//! 2. Get the current epoch and last committed epochs
//! 3. Get validator set information
//! 4. Sign a message
//! 5. Retrieve aggregation proofs by request ID
//! 6. Get individual signatures by request ID
//! 7. Get aggregation proofs by epoch
//! 8. Get signatures by epoch
//! 9. Get validator by key
//! 10. Get local validator information
//! 11. Get signature request IDs by epoch
//! 12. Get signature requests by epoch
//! 13. Stream signatures in real-time
//! 14. Stream aggregation proofs in real-time
//! 15. Stream validator set changes in real-time

use std::env;
use symbiotic_relay_client::generated::api::proto::v1::{
    GetAggregationProofRequest, GetAggregationProofsByEpochRequest, GetCurrentEpochRequest,
    GetLastAllCommittedRequest, GetLocalValidatorRequest, GetSignaturesRequest,
    GetSignaturesByEpochRequest, GetSignatureRequestIDsByEpochRequest,
    GetSignatureRequestsByEpochRequest, GetValidatorByKeyRequest, GetValidatorSetRequest,
    ListenProofsRequest, ListenSignaturesRequest, ListenValidatorSetRequest, SignMessageRequest,
    symbiotic_api_service_client::SymbioticApiServiceClient,
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

    /// Get the last all committed epochs for all chains.
    pub async fn get_last_all_committed(
        &mut self,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetLastAllCommittedResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetLastAllCommittedRequest {});
        self.client.get_last_all_committed(request).await
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
        request_id: String,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetAggregationProofResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetAggregationProofRequest { request_id });
        self.client.get_aggregation_proof(request).await
    }

    /// Get individual signatures for a request.
    pub async fn get_signatures(
        &mut self,
        request_id: String,
    ) -> Result<
        tonic::Response<symbiotic_relay_client::generated::api::proto::v1::GetSignaturesResponse>,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetSignaturesRequest { request_id });
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

    /// Get aggregation proofs by epoch.
    pub async fn get_aggregation_proofs_by_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetAggregationProofsByEpochResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetAggregationProofsByEpochRequest { epoch });
        self.client.get_aggregation_proofs_by_epoch(request).await
    }

    /// Get signatures by epoch.
    pub async fn get_signatures_by_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetSignaturesByEpochResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetSignaturesByEpochRequest { epoch });
        self.client.get_signatures_by_epoch(request).await
    }

    /// Get all signature request IDs by epoch.
    pub async fn get_signature_request_ids_by_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetSignatureRequestIDsByEpochResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetSignatureRequestIDsByEpochRequest { epoch });
        self.client
            .get_signature_request_i_ds_by_epoch(request)
            .await
    }

    /// Get all signature requests by epoch.
    pub async fn get_signature_requests_by_epoch(
        &mut self,
        epoch: u64,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetSignatureRequestsByEpochResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetSignatureRequestsByEpochRequest { epoch });
        self.client.get_signature_requests_by_epoch(request).await
    }

    /// Get validator by key.
    pub async fn get_validator_by_key(
        &mut self,
        epoch: Option<u64>,
        key_tag: u32,
        on_chain_key: Vec<u8>,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetValidatorByKeyResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetValidatorByKeyRequest {
            epoch,
            key_tag,
            on_chain_key: on_chain_key.into(),
        });
        self.client.get_validator_by_key(request).await
    }

    /// Get local validator information.
    pub async fn get_local_validator(
        &mut self,
        epoch: Option<u64>,
    ) -> Result<
        tonic::Response<
            symbiotic_relay_client::generated::api::proto::v1::GetLocalValidatorResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(GetLocalValidatorRequest { epoch });
        self.client.get_local_validator(request).await
    }

    /// Listen to signatures stream in real-time.
    pub async fn listen_signatures(
        &mut self,
        start_epoch: Option<u64>,
    ) -> Result<
        tonic::Streaming<
            symbiotic_relay_client::generated::api::proto::v1::ListenSignaturesResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(ListenSignaturesRequest { start_epoch });
        let response = self.client.listen_signatures(request).await?;
        Ok(response.into_inner())
    }

    /// Listen to aggregation proofs stream in real-time.
    pub async fn listen_proofs(
        &mut self,
        start_epoch: Option<u64>,
    ) -> Result<
        tonic::Streaming<symbiotic_relay_client::generated::api::proto::v1::ListenProofsResponse>,
        tonic::Status,
    > {
        let request = tonic::Request::new(ListenProofsRequest { start_epoch });
        let response = self.client.listen_proofs(request).await?;
        Ok(response.into_inner())
    }

    /// Listen to validator set changes stream in real-time.
    pub async fn listen_validator_set(
        &mut self,
        start_epoch: Option<u64>,
    ) -> Result<
        tonic::Streaming<
            symbiotic_relay_client::generated::api::proto::v1::ListenValidatorSetResponse,
        >,
        tonic::Status,
    > {
        let request = tonic::Request::new(ListenValidatorSetRequest { start_epoch });
        let response = self.client.listen_validator_set(request).await?;
        Ok(response.into_inner())
    }
}

/// Main example function demonstrating client usage.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize client
    let server_url = env::var("RELAY_SERVER_URL").unwrap_or_else(|_| "localhost:8080".to_string());
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
    println!("\n=== Calculate Last Committed Epoch ===");
    let mut suggested_epoch = 0u64;
    let epoch_infos_response = client.get_last_all_committed().await?;
    let epoch_infos_data = epoch_infos_response.into_inner();

    for (_, info) in epoch_infos_data.epoch_infos.iter() {
        if suggested_epoch == 0 || info.last_committed_epoch < suggested_epoch {
            suggested_epoch = info.last_committed_epoch;
        }
    }
    println!("Last committed epoch: {}", suggested_epoch);

    // Example 3: Get validator set
    println!("\n=== Getting Validator Set ===");
    let validator_set = client.get_validator_set(None).await?;
    let validator_set_data = validator_set.into_inner();
    if let Some(vs) = &validator_set_data.validator_set {
        println!("Validator set version: {}", vs.version);
        println!("Epoch: {}", vs.epoch);
        println!("Status: {:?}", vs.status());
        println!("Number of validators: {}", vs.validators.len());
        println!("Quorum threshold: {}", vs.quorum_threshold);
    }

    // Display some validator details
    if let Some(vs) = &validator_set_data.validator_set {
        if let Some(first_validator) = vs.validators.first() {
            println!("First validator operator: {}", first_validator.operator);
            println!(
                "First validator voting power: {}",
                first_validator.voting_power
            );
            println!("First validator is active: {}", first_validator.is_active);
            println!("First validator keys count: {}", first_validator.keys.len());
        }
    }

    // Example 4: Sign a message
    println!("\n=== Signing a Message ===");
    let message_to_sign = "Hello, Symbiotic!".as_bytes().to_vec();
    let key_tag = 15;

    let sign_response = client.sign_message(key_tag, message_to_sign, None).await?;
    let sign_data = sign_response.into_inner();
    println!("Request ID: {}", sign_data.request_id);
    println!("Epoch: {}", sign_data.epoch);

    // Example 5: Get aggregation proof (this might fail if signing is not complete)
    println!("\n=== Getting Aggregation Proof ===");
    match client
        .get_aggregation_proof(sign_data.request_id.clone())
        .await
    {
        Ok(proof_response) => {
            let proof_data = proof_response.into_inner();
            if let Some(proof) = proof_data.aggregation_proof {
                println!("Request ID: {}", proof.request_id);
                println!("Proof length: {} bytes", proof.proof.len());
                println!("Message hash length: {} bytes", proof.message_hash.len());
            }
        }
        Err(e) => {
            println!("Could not get aggregation proof yet: {}", e.message());
        }
    }

    // Example 6: Get individual signatures
    println!("\n=== Getting Individual Signatures ===");
    match client.get_signatures(sign_data.request_id.clone()).await {
        Ok(signatures_response) => {
            let signatures_data = signatures_response.into_inner();
            println!("Number of signatures: {}", signatures_data.signatures.len());

            for (index, signature) in signatures_data.signatures.iter().enumerate() {
                println!("Signature {}:", index + 1);
                println!("  - Request ID: {}", signature.request_id);
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

    // Example 7: Get aggregation proofs by epoch
    println!("\n=== Getting Aggregation Proofs by Epoch ===");
    let current_epoch = epoch_data.epoch;
    match client.get_aggregation_proofs_by_epoch(current_epoch).await {
        Ok(proofs_response) => {
            let proofs_data = proofs_response.into_inner();
            println!(
                "Number of proofs for epoch {}: {}",
                current_epoch,
                proofs_data.aggregation_proofs.len()
            );
            for (index, proof) in proofs_data.aggregation_proofs.iter().take(3).enumerate() {
                println!("Proof {}:", index + 1);
                println!("  - Request ID: {}", proof.request_id);
                println!("  - Proof length: {} bytes", proof.proof.len());
                println!("  - Message hash length: {} bytes", proof.message_hash.len());
            }
        }
        Err(e) => {
            println!("Could not get aggregation proofs: {}", e.message());
        }
    }

    // Example 8: Get signatures by epoch
    println!("\n=== Getting Signatures by Epoch ===");
    match client.get_signatures_by_epoch(current_epoch).await {
        Ok(signatures_response) => {
            let signatures_data = signatures_response.into_inner();
            println!(
                "Number of signatures for epoch {}: {}",
                current_epoch,
                signatures_data.signatures.len()
            );
        }
        Err(e) => {
            println!("Could not get signatures by epoch: {}", e.message());
        }
    }

    // Example 9: Get validator by key
    println!("\n=== Getting Validator by Key ===");
    if let Some(vs) = &validator_set_data.validator_set {
        if let Some(first_validator) = vs.validators.first() {
            if let Some(first_key) = first_validator.keys.first() {
                match client
                    .get_validator_by_key(None, first_key.tag, first_key.payload.to_vec())
                    .await
                {
                    Ok(validator_response) => {
                        let validator = validator_response.into_inner().validator;
                        if let Some(val) = validator {
                            println!("Found validator operator: {}", val.operator);
                            println!("Voting power: {}", val.voting_power);
                            println!("Is active: {}", val.is_active);
                        }
                    }
                    Err(e) => {
                        println!("Could not get validator by key: {}", e.message());
                    }
                }
            }
        }
    }

    // Example 10: Get local validator
    println!("\n=== Getting Local Validator ===");
    match client.get_local_validator(None).await {
        Ok(local_validator_response) => {
            let local_validator = local_validator_response.into_inner().validator;
            if let Some(val) = local_validator {
                println!("Local validator operator: {}", val.operator);
                println!("Voting power: {}", val.voting_power);
                println!("Number of keys: {}", val.keys.len());
            }
        }
        Err(e) => {
            println!("Could not get local validator: {}", e.message());
        }
    }

    // Example 11: Get signature request IDs by epoch
    println!("\n=== Getting Signature Request IDs by Epoch ===");
    match client
        .get_signature_request_ids_by_epoch(current_epoch)
        .await
    {
        Ok(ids_response) => {
            let ids_data = ids_response.into_inner();
            println!(
                "Number of signature request IDs for epoch {}: {}",
                current_epoch,
                ids_data.request_ids.len()
            );
            for (index, request_id) in ids_data.request_ids.iter().take(5).enumerate() {
                println!("  {}: {}", index + 1, request_id);
            }
        }
        Err(e) => {
            println!("Could not get signature request IDs: {}", e.message());
        }
    }

    // Example 12: Get signature requests by epoch
    println!("\n=== Getting Signature Requests by Epoch ===");
    match client.get_signature_requests_by_epoch(current_epoch).await {
        Ok(requests_response) => {
            let requests_data = requests_response.into_inner();
            println!(
                "Number of signature requests for epoch {}: {}",
                current_epoch,
                requests_data.signature_requests.len()
            );
            for (index, request) in requests_data.signature_requests.iter().take(3).enumerate() {
                println!("Signature Request {}:", index + 1);
                println!("  - Request ID: {}", request.request_id);
                println!("  - Key tag: {}", request.key_tag);
                println!("  - Message length: {} bytes", request.message.len());
                println!("  - Required epoch: {}", request.required_epoch);
            }
        }
        Err(e) => {
            println!("Could not get signature requests: {}", e.message());
        }
    }

    // Example 13: Listen to signatures stream
    println!("\n=== Listening to Signatures Stream ===");
    println!("Starting signature stream (will show first 3 signatures)...");
    match client.listen_signatures(Some(current_epoch)).await {
        Ok(mut stream) => {
            let mut count = 0;
            while let Some(signature_response) = stream.next().await {
                match signature_response {
                    Ok(sig_data) => {
                        println!("Received signature:");
                        println!("  - Request ID: {}", sig_data.request_id);
                        println!("  - Epoch: {}", sig_data.epoch);
                        if let Some(signature) = sig_data.signature {
                            println!("  - Signature request ID: {}", signature.request_id);
                            println!("  - Signature length: {} bytes", signature.signature.len());
                        }
                        count += 1;
                        if count >= 3 {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Stream error: {}", e.message());
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not start signatures stream: {}", e.message());
        }
    }

    // Example 14: Listen to proofs stream
    println!("\n=== Listening to Proofs Stream ===");
    println!("Starting proofs stream (will show first 3 proofs)...");
    match client.listen_proofs(Some(current_epoch)).await {
        Ok(mut stream) => {
            let mut count = 0;
            while let Some(proof_response) = stream.next().await {
                match proof_response {
                    Ok(proof_data) => {
                        println!("Received proof:");
                        println!("  - Request ID: {}", proof_data.request_id);
                        println!("  - Epoch: {}", proof_data.epoch);
                        if let Some(proof) = proof_data.aggregation_proof {
                            println!("  - Proof request ID: {}", proof.request_id);
                            println!("  - Proof length: {} bytes", proof.proof.len());
                        }
                        count += 1;
                        if count >= 3 {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Stream error: {}", e.message());
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not start proofs stream: {}", e.message());
        }
    }

    // Example 15: Listen to validator set changes stream
    println!("\n=== Listening to Validator Set Changes Stream ===");
    println!("Starting validator set stream (will show first 2 updates)...");
    match client.listen_validator_set(Some(current_epoch)).await {
        Ok(mut stream) => {
            let mut count = 0;
            while let Some(validator_set_response) = stream.next().await {
                match validator_set_response {
                    Ok(vs_data) => {
                        if let Some(vs) = vs_data.validator_set {
                            println!("Received validator set update:");
                            println!("  - Epoch: {}", vs.epoch);
                            println!("  - Number of validators: {}", vs.validators.len());
                            println!("  - Status: {:?}", vs.status());
                        }
                        count += 1;
                        if count >= 2 {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Stream error: {}", e.message());
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not start validator set stream: {}", e.message());
        }
    }

    println!("\nExample completed");
    Ok(())
}
