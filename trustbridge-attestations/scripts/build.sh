#!/bin/bash
set -e

echo "🔨 Building TrustBridge Attestations"
echo ""

# Build contracts
echo "📦 Building Soroban contracts..."
cd contracts/attestation-registry
cargo build --target wasm32-unknown-unknown --release
cd ../..

echo "✅ Contracts built successfully"
echo ""

# Build frontend
echo "🌐 Building frontend..."
cd frontend
npm run build
cd ..

echo "✅ Frontend built successfully"
echo ""
echo "🎉 Build complete!"
