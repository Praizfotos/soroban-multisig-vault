# Quick Start Guide

Get started with Soroban Multi-Sig Treasury Vault in 5 minutes.

## What You'll Build

A multi-signature treasury where:
- 3 signers control the funds
- 2 out of 3 approvals required for any action
- Complete audit trail on Stellar blockchain

## Prerequisites

- Rust installed
- Node.js 18+
- Docker (optional)

## Installation

```bash
# Clone repository
git clone https://github.com/your-org/soroban-multisig-vault.git
cd soroban-multisig-vault

# Install dependencies
make install

# Start local environment
make dev
```

## Create Your First Treasury

### 1. Connect Wallet

Visit `http://localhost:3000` and click "Connect Wallet". Install Freighter if needed.

### 2. Create Treasury

```typescript
// In the web UI or using CLI
Treasury Name: "My DAO Treasury"
Signers: 
  - Your address
  - Signer 2 address
  - Signer 3 address
Threshold: 2 of 3
```

### 3. Fund Treasury

```typescript
// Deposit 1000 XLM
Amount: 1000 XLM
```

### 4. Create Proposal

```typescript
Type: Transfer
Recipient: [address]
Amount: 100 XLM
Expiration: 7 days
```

### 5. Vote and Execute

- Share proposal link with signers
- Each signer votes approve/reject
- Automatically executes when threshold met

## Next Steps

- [Read full documentation](./docs/ARCHITECTURE.md)
- [Deploy to testnet](./docs/DEPLOYMENT.md)
- [Security best practices](./docs/SECURITY.md)
- [API integration](./docs/API.md)

## Common Use Cases

### DAO Treasury
```
Signers: 5 core members
Threshold: 3 of 5
```

### Startup Operations
```
Signers: Founder, CFO, CTO
Threshold: 2 of 3
```

### Grant Program
```
Signers: 7 committee members
Threshold: 4 of 7
```

## Support

- GitHub Issues: Report bugs
- Discord: Community chat
- Docs: Comprehensive guides

---

Built for the Stellar ecosystem 🚀
