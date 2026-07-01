#!/bin/bash

set -e

echo "⚠️  MAINNET DEPLOYMENT ⚠️"
echo ""
echo "This will deploy contracts to Stellar Mainnet."
echo "Make sure you have:"
echo "  1. Audited all smart contracts"
echo "  2. Run comprehensive tests"
echo "  3. Funded deployment account with XLM"
echo "  4. Backed up deployment keys"
echo ""
read -p "Continue? (yes/no): " -r
echo

if [[ ! $REPLY =~ ^yes$ ]]; then
    echo "Deployment cancelled"
    exit 1
fi

# Check prerequisites
if ! command -v soroban &> /dev/null; then
    echo "❌ Error: soroban CLI not found"
    exit 1
fi

# Set network
NETWORK="mainnet"
NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
RPC_URL="https://soroban-mainnet.stellar.org"

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
echo "🔧 Deploying Treasury contract to MAINNET..."
TREASURY_WASM="target/wasm32-unknown-unknown/release/treasury_optimized.wasm"
TREASURY_ID=$(soroban contract deploy \
    --wasm $TREASURY_WASM \
    --network $NETWORK \
    --source-account mainnet)

echo "Treasury Contract ID: $TREASURY_ID"

# Deploy Governance contract
echo ""
echo "🔧 Deploying Governance contract to MAINNET..."
GOVERNANCE_WASM="target/wasm32-unknown-unknown/release/governance_optimized.wasm"
GOVERNANCE_ID=$(soroban contract deploy \
    --wasm $GOVERNANCE_WASM \
    --network $NETWORK \
    --source-account mainnet)

echo "Governance Contract ID: $GOVERNANCE_ID"

# Deploy Registry contract
echo ""
echo "🔧 Deploying Registry contract to MAINNET..."
REGISTRY_WASM="target/wasm32-unknown-unknown/release/registry_optimized.wasm"
REGISTRY_ID=$(soroban contract deploy \
    --wasm $REGISTRY_WASM \
    --network $NETWORK \
    --source-account mainnet)

echo "Registry Contract ID: $REGISTRY_ID"

# Save contract IDs
cd ..
cat > .env.mainnet << EOF
# MAINNET Deployed Contract IDs - $(date)
SOROBAN_NETWORK=$NETWORK
TREASURY_CONTRACT_ID=$TREASURY_ID
GOVERNANCE_CONTRACT_ID=$GOVERNANCE_ID
REGISTRY_CONTRACT_ID=$REGISTRY_ID
EOF

echo ""
echo "✅ MAINNET deployment complete!"
echo ""
echo "⚠️  IMPORTANT: Backup this information securely!"
echo ""
echo "Contract IDs saved to .env.mainnet"
echo ""
echo "Treasury:    $TREASURY_ID"
echo "Governance:  $GOVERNANCE_ID"
echo "Registry:    $REGISTRY_ID"
echo ""
echo "🔐 Next steps:"
echo "  1. Verify contracts on Stellar Expert"
echo "  2. Update production environment variables"
echo "  3. Test with small amounts first"
echo "  4. Document contract addresses publicly"
