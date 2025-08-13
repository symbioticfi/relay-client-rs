#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. && pwd)"
PROTO_DIR="$ROOT_DIR/api/proto/v1"

rm -rf "$ROOT_DIR/api" "$ROOT_DIR/src/generated"
mkdir -p "$PROTO_DIR"

curl -sfL https://raw.githubusercontent.com/symbioticfi/relay/dev/api/proto/v1/api.proto -o "$PROTO_DIR/api.proto"

cd "$ROOT_DIR"
buf format -w
buf lint
buf generate

# Ensure mod.rs exists for stable imports if plugins didn't create it
if [ ! -f src/generated/mod.rs ]; then
  mkdir -p src/generated
  cat > src/generated/mod.rs << 'EOF'
//! Generated gRPC client code from protocol buffer definitions.
//!
//! This module contains the auto-generated Rust code from the protocol buffer
//! definitions for the Symbiotic Relay API.

pub mod api {
    pub mod proto {
        pub mod v1 {
            // Include the generated prost types and client code
            include!("api.proto.v1.rs");
        }
    }
}
EOF
fi
