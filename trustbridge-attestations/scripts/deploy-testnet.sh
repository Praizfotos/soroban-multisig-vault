#!/bin/bash
set -e

echo "🚀 Deploying TrustBridge Attestations to Stellar Testnet"
echo ""

# Check if soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo "❌ Error: soroban CLI is not installed"
    echo "Install it with: cargo install --locked soroban-cli"
    exit 1
fi

# Build the contract
echo "📦 Building contract..."
cd contracts/attestation-registry
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Optimize the WASM
echo "⚡ Optimizing WASM..."
soroban contract optimize \
    --wasm target/wasm32-unknown-unknown/release/attestation_registry.wasm

# Deploy to testnet
echo "🌐 Deploying to testnet..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/attestation_registry.wasm \
    --source admin \
    --network testnet)

echo ""
echo "✅ Deployment successful!"
echo "Contract ID: $CONTRACT_ID"
echo ""
echo "📝 Save this contract ID to your .env file:"
echo "NEXT_PUBLIC_CONTRACT_ID=$CONTRACT_ID"
echo ""

# Initialize the contract
echo "🔧 Initializing contract..."
ADMIN_ADDRESS=$(soroban keys address admin)

soroban contract invoke \
    --id $CONTRACT_ID \
    --source admin \
    --network testnet \
    -- initialize \
    --admin $ADMIN_ADDRESS

echo ""
echo "✅ Contract initialized with admin: $ADMIN_ADDRESS"
echo ""
echo "🎉 Deployment complete!"
