# Soroban Multi-Sig Treasury Vault

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Stellar](https://img.shields.io/badge/Stellar-Soroban-purple.svg)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)

A production-ready, secure treasury management protocol built on Stellar and Soroban. Enable organizations, DAOs, startups, and communities to manage funds through configurable multi-signature approval workflows.

## 🎯 Overview

Soroban Multi-Sig Treasury Vault provides enterprise-grade treasury infrastructure where:

- **No single wallet** can move funds
- **Every action** requires multi-signature approval
- **Complete transparency** through on-chain audit trails
- **Configurable governance** with N-of-M approval thresholds
- **Emergency controls** for security incidents

## 🔥 Features

### Multi-Signature Treasury Management
- Create treasuries with customizable signer sets
- Support 2-of-3, 3-of-5, 4-of-7, or any N-of-M configuration
- Secure fund custody with collective decision-making

### Proposal-Based Governance
- Transfer funds with multi-sig approval
- Add/remove signers through governance
- Update approval thresholds
- Emergency pause/resume controls

### Asset Management
- Support XLM and Stellar assets
- Track deposits and balances
- Transparent transaction history

### Security Features
- Authorization guards
- Replay protection
- Proposal expiration
- Emergency pause mechanism
- Input validation

### Event Emission
- Complete audit trail
- Real-time notifications
- Integration-friendly

## 🏗️ Architecture

```
soroban-multisig-vault/
├── contracts/          # Soroban smart contracts (Rust)
├── frontend/           # Next.js web application
├── backend/            # Node.js indexer and API
├── database/           # PostgreSQL schemas
├── tests/              # Integration and security tests
└── infrastructure/     # DevOps and deployment
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.75+
- Soroban CLI
- Node.js 18+
- PostgreSQL 14+
- Docker & Docker Compose

### Installation

```bash
# Clone repository
git clone https://github.com/your-org/soroban-multisig-vault.git
cd soroban-multisig-vault

# Install dependencies
make install

# Build contracts
make build-contracts

# Run tests
make test

# Start local development
make dev
```

## 📚 Documentation

- [Architecture Guide](./docs/ARCHITECTURE.md)
- [Smart Contract Documentation](./docs/CONTRACTS.md)
- [API Reference](./docs/API.md)
- [Security Model](./docs/SECURITY.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)

## 🔒 Security

This protocol implements multiple security layers:

- Multi-signature requirement enforcement
- Proposal expiration windows
- Emergency pause capabilities
- Threshold validation
- Replay attack prevention

See [SECURITY.md](./docs/SECURITY.md) for detailed threat analysis.

## 🧪 Testing

```bash
# Run all tests
make test

# Unit tests only
make test-unit

# Integration tests
make test-integration

# Security tests
make test-security

# Coverage report
make coverage
```

## 📦 Deployment

### Testnet

```bash
make deploy-testnet
```

### Mainnet

```bash
make deploy-mainnet
```

See [DEPLOYMENT.md](./docs/DEPLOYMENT.md) for detailed instructions.

## 🤝 Contributing

Contributions welcome! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) first.

## 📄 License

MIT License - see [LICENSE](./LICENSE) file for details.

## 🌟 Use Cases

- **DAOs**: Decentralized treasury management
- **Startups**: Secure multi-sig company funds
- **Grant Programs**: Transparent fund distribution
- **Communities**: Collective resource management
- **Foundations**: Multi-party oversight

## 🛠️ Built With

- **Soroban SDK** - Smart contract framework
- **Rust** - Contract implementation
- **Next.js** - Frontend framework
- **TypeScript** - Type safety
- **PostgreSQL** - Data persistence
- **Docker** - Containerization

## 📞 Support

- GitHub Issues: [Report bugs](https://github.com/your-org/soroban-multisig-vault/issues)
- Discord: [Join community](https://discord.gg/stellar)
- Documentation: [Read docs](./docs)

---

Built with ❤️ for the Stellar ecosystem
