#!/bin/bash

set -e

echo "🚀 Deploying to Stellar Testnet"

# Check prerequisites
if ! command -v soroban &> /dev/null; then
    echo "❌ Error: soroban CLI not found"
    echo "Install from: https://soroban.stellar.org/docs/getting-started/setup"
    exit 1
fi

if ! command -v stellar &> /dev/null; then
    echo "⚠️  Warning: stellar CLI not found"
fi

# Set network
NETWORK="testnet"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
RPC_URL="https://soroban-testnet.stellar.org"

echo "Network: $NETWORK"
echo "RPC URL: $RPC_URL"

# Build contracts
echo ""
echo "📦 Building contracts..."
cd contracts
cargo build --target wasm32-unknown-unknown --release

# Optimize WASMs
echo ""
echo "⚙️  Optimizing WASM..."
soroban contract optimize \
    --wasm target/wasm32-unknown-unknown/release/treasury.wasm \
    --wasm-out target/wasm32-unknown-unknown/release/treasury_optimized.wasm

soroban contract optimize \
    --wasm target/wasm32-unknown-unknown/release/governance.wasm \
    --wasm-out target/wasm32-unknown-unknown/release/governance_optimized.wasm

soroban contract optimize \
    --wasm target/wasm32-unknown-unknown/release/registry.wasm \
    --wasm-out target/wasm32-unknown-unknown/release/registry_optimized.wasm

# Deploy Treasury contract
echo ""
echo "🔧 Deploying Treasury contract..."
TREASURY_WASM="target/wasm32-unknown-unknown/release/treasury_optimized.wasm"
TREASURY_ID=$(soroban contract deploy \
    --wasm $TREASURY_WASM \
    --network $NETWORK \
    --source-account default)

echo "Treasury Contract ID: $TREASURY_ID"

# Deploy Governance contract
echo ""
echo "🔧 Deploying Governance contract..."
GOVERNANCE_WASM="target/wasm32-unknown-unknown/release/governance_optimized.wasm"
GOVERNANCE_ID=$(soroban contract deploy \
    --wasm $GOVERNANCE_WASM \
    --network $NETWORK \
    --source-account default)

echo "Governance Contract ID: $GOVERNANCE_ID"

# Deploy Registry contract
echo ""
echo "🔧 Deploying Registry contract..."
REGISTRY_WASM="target/wasm32-unknown-unknown/release/registry_optimized.wasm"
REGISTRY_ID=$(soroban contract deploy \
    --wasm $REGISTRY_WASM \
    --network $NETWORK \
    --source-account default)

echo "Registry Contract ID: $REGISTRY_ID"

# Save contract IDs
cd ..
cat > .env.deployed << EOF
# Deployed Contract IDs - $(date)
SOROBAN_NETWORK=$NETWORK
TREASURY_CONTRACT_ID=$TREASURY_ID
GOVERNANCE_CONTRACT_ID=$GOVERNANCE_ID
REGISTRY_CONTRACT_ID=$REGISTRY_ID
EOF

echo ""
echo "✅ Deployment complete!"
echo ""
echo "Contract IDs saved to .env.deployed"
echo ""
echo "Update your environment files:"
echo "  - frontend/.env"
echo "  - backend/.env"
echo ""
echo "Treasury:    $TREASURY_ID"
echo "Governance:  $GOVERNANCE_ID"
echo "Registry:    $REGISTRY_ID"
