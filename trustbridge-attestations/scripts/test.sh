#!/bin/bash
set -e

echo "🧪 Running TrustBridge Attestations tests"
echo ""

# Test contracts
echo "📝 Testing Soroban contracts..."
cd contracts/attestation-registry
cargo test
cd ../..

echo "✅ All contract tests passed"
echo ""
echo "🎉 Tests complete!"
