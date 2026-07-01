# Architecture

## System Overview

TrustBridge Attestations is a decentralized reputation and verification system built on Stellar's Soroban smart contract platform. The system enables trusted entities to issue verifiable, on-chain attestations about subjects in the ecosystem.

## Components

### 1. Smart Contract Layer

The core Soroban contract provides:

- **Storage Management**: Persistent storage for attestations, trusted issuers, and admin data
- **Authorization**: Role-based access control for admin and issuer operations
- **Business Logic**: Attestation lifecycle management (issue, revoke, query)
- **Event Emission**: Audit trail through contract events

### 2. Frontend Layer

Next.js application providing:

- **Wallet Integration**: Freighter wallet connectivity
- **User Interface**: Attestation explorer and issuance forms
- **Contract Interaction**: Transaction building and submission
- **Data Display**: Query and display attestation data

### 3. Deployment Scripts

Automation for:

- Contract compilation and optimization
- Testnet/mainnet deployment
- Contract initialization
- Testing and validation

## Data Model

### Attestation

```rust
pub struct Attestation {
    pub id: u64,                    // Unique identifier
    pub issuer: Address,            // Who issued it
    pub subject: Address,           // Who it's about
    pub attestation_type: String,   // Type/category
    pub data: String,               // Attestation content
    pub timestamp: u64,             // When it was issued
    pub revoked: bool,              // Revocation status
}
```

### Storage Keys

```rust
pub enum DataKey {
    Admin,                          // Contract admin address
    TrustedIssuer(Address),         // Is address a trusted issuer?
    Attestation(u64),               // Attestation by ID
    AttestationCounter,             // Global attestation counter
    SubjectAttestations(Address),   // List of attestation IDs for subject
    IssuerAttestations(Address),    // List of attestation IDs by issuer
}
```

## Access Control

### Admin Role

- Initialize contract
- Add trusted issuers
- Remove trusted issuers
- Transfer admin rights

### Trusted Issuer Role

- Issue attestations
- Revoke own attestations
- Query attestations

### Public Access

- View attestations
- Query by subject or issuer
- Check issuer trust status

## Operational Flows

### Initialization Flow

1. Deploy contract to network
2. Admin calls `initialize(admin_address)`
3. Contract stores admin and sets counter to 0
4. Contract emits `initialized` event

### Issuer Onboarding

1. Admin calls `add_trusted_issuer(issuer_address)`
2. Contract requires admin authorization
3. Contract checks issuer not already trusted
4. Contract stores issuer trust status
5. Contract emits `trusted_issuer_added` event

### Attestation Issuance

1. Issuer calls `issue_attestation(issuer, input)`
2. Contract requires issuer authorization
3. Contract validates issuer is trusted
4. Contract validates input data
5. Contract increments counter
6. Contract creates attestation with timestamp
7. Contract stores attestation
8. Contract updates subject and issuer indexes
9. Contract emits `attestation_issued` event
10. Returns attestation ID

### Attestation Revocation

1. Issuer calls `revoke_attestation(issuer, id)`
2. Contract requires issuer authorization
3. Contract loads attestation by ID
4. Contract validates issuer owns attestation
5. Contract validates not already revoked
6. Contract updates attestation status
7. Contract emits `attestation_revoked` event

### Query Operations

Queries are read-only and don't require authorization:

- `get_attestation(id)` - Single attestation lookup
- `get_attestations_by_subject(address)` - All attestations for subject
- `get_attestations_by_issuer(address)` - All attestations by issuer
- `is_trusted_issuer(address)` - Check trust status
- `get_info()` - Contract metadata

## Storage Design

### Instance Storage

Used for contract-level data:
- Admin address
- Attestation counter

### Persistent Storage

Used for long-lived data:
- Attestations (by ID)
- Trusted issuer status (by address)
- Subject attestation indexes (by address)
- Issuer attestation indexes (by address)

## Event Schema

All operations emit events for off-chain indexing:

```rust
// Initialization
("initialized", admin_address)

// Issuer management
("trusted_issuer_added", issuer_address)
("trusted_issuer_removed", issuer_address)

// Attestations
("attestation_issued", (id, issuer, subject))
("attestation_revoked", (id, issuer))

// Admin transfer
("admin_transferred", (old_admin, new_admin))
```

## Scalability Considerations

### On-Chain Efficiency

- Minimal storage per attestation
- Index-based queries for subject/issuer lookups
- No loops in contract logic
- Optimized WASM binary

### Off-Chain Indexing

For production systems, consider:
- Indexer service listening to contract events
- Database for fast queries across large datasets
- API layer for complex search operations
- Caching layer for frequently accessed data

## Security Architecture

### Authorization Model

- All write operations require caller authentication
- Admin operations check caller is admin
- Issuer operations check caller is trusted issuer
- Revocation checks caller is original issuer

### Data Integrity

- Attestations are immutable (except revoked flag)
- Timestamps are set by ledger, not caller
- IDs are sequential and controlled by contract
- No external data dependencies

### Attack Mitigations

- Replay protection: Stellar transaction nonces
- Sybil resistance: Trusted issuer whitelist
- Spam prevention: Transaction fees
- Front-running: Deterministic outcomes
- DoS: Bounded operations, no loops

## Future Enhancements

### Potential Extensions

1. **Attestation Types Registry** - Standard types with schemas
2. **Delegation** - Allow issuers to delegate authority
3. **Expiration** - Time-limited attestations
4. **Privacy** - Zero-knowledge proof integration
5. **Multi-Sig Issuance** - Require multiple issuers
6. **Attestation Updates** - Update data without revoking
7. **Cross-Chain Bridge** - Verify attestations on other chains

### Integration Patterns

- **DID Integration** - Map attestations to decentralized identifiers
- **IPFS Storage** - Store large attestation data off-chain
- **Oracle Integration** - Automated attestations from external data
- **Governance** - Community voting on trusted issuers

## Deployment Architecture

### Testnet

```
Developer → Freighter Wallet → Soroban Testnet → Contract
                                      ↓
                              Event Stream → Logs
```

### Production

```
User → Frontend (Next.js) → Freighter → Soroban Mainnet → Contract
                                              ↓
                                        Event Stream
                                              ↓
                                    Indexer Service
                                              ↓
                                         Database
                                              ↓
                                    API Server ← Frontend Query
```

## Testing Strategy

### Unit Tests

- Individual function behavior
- Error conditions
- Authorization checks
- Edge cases

### Integration Tests

- Multi-operation workflows
- State transitions
- Event emissions
- Query operations

### Security Tests

- Authorization bypass attempts
- Invalid input handling
- Reentrancy scenarios
- Resource exhaustion

---

This architecture provides a solid foundation for decentralized attestations while remaining flexible for future enhancements.
