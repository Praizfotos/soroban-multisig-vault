# TrustBridge Attestations

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Stellar](https://img.shields.io/badge/Stellar-Soroban-purple.svg)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)

On-chain trustless attestation registry for verifiable Stellar contributor reputation built on Soroban smart contracts.

## 🎯 Overview

TrustBridge Attestations enables trusted entities to issue verifiable, on-chain attestations about contributors, organizations, and participants in the Stellar ecosystem. Create transparent, tamper-proof reputation systems with decentralized verification.

## 🔥 Features

### Smart Contract Features
- **Trusted Issuer Registry** - Admin-managed whitelist of authorized attestation issuers
- **Attestation Issuance** - Trusted issuers can create on-chain attestations for any subject
- **Attestation Revocation** - Issuers can revoke their own attestations when needed
- **Query by Subject** - Retrieve all attestations for a specific address
- **Query by Issuer** - Retrieve all attestations issued by a specific entity
- **Admin Controls** - Transfer admin rights and manage trusted issuers
- **Event Emission** - Complete audit trail of all operations

### Frontend Features
- **Freighter Wallet Integration** - Seamless connection to Stellar wallets
- **Attestation Explorer** - View, filter, and search attestations
- **Issuance Interface** - Easy-to-use form for issuing attestations
- **Responsive Design** - Built with Tailwind CSS and dark mode support
- **TypeScript** - Full type safety across the application

### Security Features
- Authorization guards on all sensitive operations
- Only issuers can revoke their own attestations
- Admin-only trusted issuer management
- Input validation
- Comprehensive error handling

## 🏗️ Architecture

```
trustbridge-attestations/
├── contracts/
│   └── attestation-registry/     # Soroban smart contract
│       ├── src/
│       │   ├── lib.rs            # Main contract logic
│       │   ├── types.rs          # Data structures
│       │   ├── errors.rs         # Error definitions
│       │   ├── storage.rs        # Storage operations
│       │   └── test.rs           # Comprehensive tests
│       └── Cargo.toml
├── frontend/                      # Next.js application
│   ├── app/                       # App router pages
│   ├── components/                # React components
│   └── lib/                       # Utilities and contract interactions
├── scripts/                       # Deployment and build scripts
└── docs/                          # Documentation
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+ with `wasm32-unknown-unknown` target
- Soroban CLI (`cargo install soroban-cli`)
- Node.js 18+
- npm or yarn

### Installation

```bash
# Clone the repository
git clone https://github.com/Praizfotos/trustbridge-attestations.git
cd trustbridge-attestations

# Build contracts
cd contracts/attestation-registry
cargo build --target wasm32-unknown-unknown --release
cd ../..

# Install frontend dependencies
cd frontend
npm install
cd ..
```

### Running Tests

```bash
# Test smart contracts
cd contracts/attestation-registry
cargo test
cargo fmt --check
cargo clippy -- -D warnings
cd ../..

# Or use the test script
./scripts/test.sh
```

### Local Development

```bash
# Start frontend development server
cd frontend
npm run dev
```

Visit `http://localhost:3000` to see the application.

## 📦 Deployment

### Deploy to Testnet

```bash
# Configure Soroban CLI for testnet
soroban network add testnet \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase "Test SDF Network ; September 2015"

# Create or import an admin identity
soroban keys generate admin

# Run deployment script
./scripts/deploy-testnet.sh
```

The script will:
1. Build and optimize the contract
2. Deploy to Stellar testnet
3. Initialize the contract with your admin address
4. Output the contract ID for your frontend configuration

### Configure Frontend

Create a `.env.local` file in the `frontend/` directory:

```env
NEXT_PUBLIC_CONTRACT_ID=your_contract_id_here
NEXT_PUBLIC_NETWORK=TESTNET
```

## 🔒 Smart Contract Interface

### Initialize

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

Initialize the contract with an admin address. Can only be called once.

### Add Trusted Issuer

```rust
pub fn add_trusted_issuer(env: Env, issuer: Address) -> Result<(), Error>
```

Add an address to the trusted issuer whitelist. Admin only.

### Issue Attestation

```rust
pub fn issue_attestation(
    env: Env,
    issuer: Address,
    input: AttestationInput
) -> Result<u64, Error>
```

Issue a new attestation. Trusted issuer only. Returns the attestation ID.

### Revoke Attestation

```rust
pub fn revoke_attestation(env: Env, issuer: Address, id: u64) -> Result<(), Error>
```

Revoke an attestation. Only the original issuer can revoke.

### Query Functions

```rust
pub fn get_attestation(env: Env, id: u64) -> Option<Attestation>
pub fn get_attestations_by_subject(env: Env, subject: Address) -> Vec<Attestation>
pub fn get_attestations_by_issuer(env: Env, issuer: Address) -> Vec<Attestation>
pub fn is_trusted_issuer(env: Env, issuer: Address) -> bool
pub fn get_info(env: Env) -> ContractInfo
```

## 📚 Documentation

- [Architecture](./docs/ARCHITECTURE.md) - System design and data flow
- [Security](./docs/SECURITY.md) - Security model and threat analysis
- [API Reference](./docs/API.md) - Complete contract interface documentation

## 🧪 Testing

The contract includes comprehensive unit tests covering:

- Initialization and admin setup
- Trusted issuer management
- Attestation issuance and validation
- Revocation controls and authorization
- Query operations
- Error handling and edge cases

Run tests with:

```bash
cd contracts/attestation-registry
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Submit a pull request

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

## 🌟 Use Cases

- **Developer Reputation** - Build verifiable profiles for open-source contributors
- **KYC/Compliance** - Issue compliance attestations from trusted verifiers
- **Credentials** - Create on-chain certificates and certifications
- **Community Badges** - Reward community participation with attestations
- **Grant Eligibility** - Verify eligibility for funding programs
- **Identity Verification** - Decentralized identity attributes

## 🛠️ Built With

- [Soroban SDK](https://soroban.stellar.org/) - Smart contract framework
- [Rust](https://www.rust-lang.org/) - Contract implementation language
- [Next.js](https://nextjs.org/) - React framework for the frontend
- [TypeScript](https://www.typescriptlang.org/) - Type-safe JavaScript
- [Tailwind CSS](https://tailwindcss.com/) - Utility-first CSS framework
- [Freighter](https://www.freighter.app/) - Stellar wallet integration

## 📞 Support

- GitHub Issues: [Report bugs or request features](https://github.com/Praizfotos/trustbridge-attestations/issues)
- Stellar Discord: [Join the community](https://discord.gg/stellar)
- Documentation: [Read the docs](./docs)

---

Built with ❤️ for the Stellar ecosystem
