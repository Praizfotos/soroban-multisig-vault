# Smart Contract Documentation

Complete reference for Soroban Multi-Sig Treasury Vault smart contracts.

## Contract Addresses

### Testnet
- Treasury: `[Deployed via scripts/deploy-testnet.sh]`
- Governance: `[Deployed via scripts/deploy-testnet.sh]`
- Registry: `[Deployed via scripts/deploy-testnet.sh]`

### Mainnet
- Treasury: `[Deployed via scripts/deploy-mainnet.sh]`
- Governance: `[Deployed via scripts/deploy-mainnet.sh]`
- Registry: `[Deployed via scripts/deploy-mainnet.sh]`

## Treasury Contract

The core multi-signature treasury contract.

### Data Structures

#### Treasury
```rust
struct Treasury {
    id: String,              // Unique identifier
    name: String,            // Human-readable name
    creator: Address,        // Creator address
    signers: Vec<Address>,   // Authorized signers
    threshold: u32,          // Approval threshold
    created_at: u64,         // Creation timestamp
    paused: bool,            // Emergency pause status
}
```

#### Proposal
```rust
struct Proposal {
    id: String,                    // Unique identifier
    treasury_id: String,           // Parent treasury
    proposal_type: ProposalType,   // Type of proposal
    proposer: Address,             // Who created it
    status: ProposalStatus,        // Current status
    approvals: Vec<Address>,       // Approval votes
    rejections: Vec<Address>,      // Rejection votes
    created_at: u64,               // Creation time
    expires_at: u32,               // Expiration ledger
    executed: bool,                // Execution status
}
```

#### ProposalType
```rust
enum ProposalType {
    Transfer {
        recipient: Address,
        asset: Address,
        amount: i128,
    },
    AddSigner {
        new_signer: Address,
    },
    RemoveSigner {
        signer: Address,
    },
    UpdateThreshold {
        new_threshold: u32,
    },
    Pause,
    Resume,
}
```

### Functions

#### initialize
Create a new treasury.

```rust
fn initialize(
    env: Env,
    treasury_id: String,
    name: String,
    creator: Address,
    signers: Vec<Address>,
    threshold: u32,
) -> Result<(), TreasuryError>
```

**Parameters**:
- `treasury_id` - Unique identifier for treasury
- `name` - Human-readable treasury name
- `creator` - Address of creator (must be in signers)
- `signers` - List of authorized signer addresses
- `threshold` - Number of approvals required (1 to signers.len())

**Requirements**:
- Creator must sign transaction
- Signers list not empty
- Threshold between 1 and number of signers
- Treasury ID must not already exist

**Emits**: `TreasuryCreated`

**Example**:
```typescript
await treasuryClient.initialize(
  "dao-treasury-1",
  "DAO Main Treasury",
  creatorAddress,
  [signer1, signer2, signer3],
  2
);
```

#### deposit
Deposit assets into treasury.

```rust
fn deposit(
    env: Env,
    treasury_id: String,
    asset: Address,
    amount: i128,
    depositor: Address,
) -> Result<(), TreasuryError>
```

**Parameters**:
- `treasury_id` - Target treasury
- `asset` - Asset contract address (or native)
- `amount` - Amount to deposit
- `depositor` - Address making deposit

**Requirements**:
- Depositor must sign transaction
- Treasury must exist and not be paused
- Depositor must have sufficient balance
- Token contract must allow transfer

**Emits**: `DepositReceived`

#### create_proposal
Create a governance proposal.

```rust
fn create_proposal(
    env: Env,
    treasury_id: String,
    proposal_id: String,
    proposer: Address,
    proposal_type: ProposalType,
    expiration_ledger: u32,
) -> Result<(), TreasuryError>
```

**Parameters**:
- `treasury_id` - Treasury this proposal belongs to
- `proposal_id` - Unique proposal identifier
- `proposer` - Address creating proposal
- `proposal_type` - Type and payload of proposal
- `expiration_ledger` - Ledger when proposal expires

**Requirements**:
- Proposer must sign transaction
- Proposer must be authorized signer
- Treasury must exist
- If treasury paused, only Resume proposals allowed
- Proposal ID must not already exist
- Expiration must be in future

**Emits**: `ProposalCreated`

**Example**:
```typescript
await treasuryClient.createProposal(
  "dao-treasury-1",
  "proposal-transfer-1",
  proposerAddress,
  {
    Transfer: {
      recipient: recipientAddress,
      asset: nativeAsset,
      amount: 1000_0000000n
    }
  },
  currentLedger + 100000
);
```

#### vote
Vote on a proposal.

```rust
fn vote(
    env: Env,
    proposal_id: String,
    voter: Address,
    approve: bool,
) -> Result<(), TreasuryError>
```

**Parameters**:
- `proposal_id` - Proposal to vote on
- `voter` - Address casting vote
- `approve` - true for approve, false for reject

**Requirements**:
- Voter must sign transaction
- Voter must be authorized signer
- Proposal must exist and not be executed
- Proposal must not be expired
- Voter must not have already voted

**Behavior**:
- Records vote (approval or rejection)
- Checks if threshold reached
- If approved and threshold met, auto-executes
- If rejected by majority, marks as rejected

**Emits**: `VoteCast`, optionally `ProposalExecuted`

**Example**:
```typescript
await treasuryClient.vote(
  "proposal-transfer-1",
  voterAddress,
  true  // approve
);
```

#### execute_proposal
Manually execute an approved proposal.

```rust
fn execute_proposal(
    env: Env,
    proposal_id: String,
) -> Result<(), TreasuryError>
```

**Parameters**:
- `proposal_id` - Proposal to execute

**Requirements**:
- Proposal must be approved
- Proposal must not be expired
- Proposal must not already be executed

**Emits**: `ProposalExecuted`, plus type-specific events

#### get_treasury
Query treasury details.

```rust
fn get_treasury(
    env: Env,
    treasury_id: String,
) -> Result<Treasury, TreasuryError>
```

#### get_proposal
Query proposal details.

```rust
fn get_proposal(
    env: Env,
    proposal_id: String,
) -> Result<Proposal, TreasuryError>
```

#### get_balance
Query treasury balance for an asset.

```rust
fn get_balance(
    env: Env,
    treasury_id: String,
    asset: Address,
) -> Result<i128, TreasuryError>
```

### Events

All events include timestamps and relevant identifiers.

#### TreasuryCreated
```rust
(
    treasury_id: String,
    name: String,
    creator: Address,
    signer_count: u32,
    threshold: u32,
)
```

#### DepositReceived
```rust
(
    treasury_id: String,
    asset: Address,
    amount: i128,
    depositor: Address,
)
```

#### ProposalCreated
```rust
(
    proposal_id: String,
    treasury_id: String,
    proposer: Address,
)
```

#### VoteCast
```rust
(
    proposal_id: String,
    voter: Address,
    approve: bool,
)
```

#### ProposalExecuted
```rust
(proposal_id: String)
```

#### SignerAdded
```rust
(
    treasury_id: String,
    signer: Address,
)
```

#### SignerRemoved
```rust
(
    treasury_id: String,
    signer: Address,
)
```

#### ThresholdUpdated
```rust
(
    treasury_id: String,
    new_threshold: u32,
)
```

#### TreasuryPaused
```rust
(treasury_id: String)
```

#### TreasuryResumed
```rust
(treasury_id: String)
```

### Error Codes

```rust
pub enum TreasuryError {
    TreasuryNotFound = 1,
    TreasuryExists = 2,
    TreasuryPaused = 3,
    TreasuryNotPaused = 4,
    Unauthorized = 10,
    InvalidSigners = 20,
    SignerExists = 21,
    SignerNotFound = 22,
    InvalidThreshold = 30,
    ProposalNotFound = 40,
    ProposalExists = 41,
    ProposalExpired = 42,
    ProposalExecuted = 43,
    ProposalNotApproved = 44,
    InvalidProposalStatus = 45,
    InvalidExpiration = 46,
    AlreadyVoted = 50,
    InsufficientBalance = 60,
    InvalidInput = 100,
}
```

## Governance Contract

Manages voting configuration and quorum checks.

### Functions

#### initialize
```rust
fn initialize(
    env: Env,
    treasury_contract: Address,
    voting_period: u32,
    execution_delay: u32,
    quorum_threshold: u32,
)
```

#### record_vote
```rust
fn record_vote(
    env: Env,
    proposal_id: String,
    voter: Address,
    vote_type: VoteType,
    voting_power: u32,
)
```

#### check_quorum
```rust
fn check_quorum(
    env: Env,
    approve_count: u32,
    reject_count: u32,
    abstain_count: u32,
    total_voting_power: u32,
) -> bool
```

## Registry Contract

Treasury discovery and metadata management.

### Functions

#### register_treasury
```rust
fn register_treasury(
    env: Env,
    treasury_id: String,
    name: String,
    contract_address: Address,
    creator: Address,
    metadata: String,
)
```

#### get_all_treasuries
```rust
fn get_all_treasuries(env: Env) -> Vec<String>
```

#### get_treasuries_by_creator
```rust
fn get_treasuries_by_creator(
    env: Env,
    creator: Address,
) -> Vec<String>
```

#### get_stats
```rust
fn get_stats(env: Env) -> RegistryStats
```

## Integration Examples

### Creating and Funding Treasury

```typescript
// 1. Initialize treasury
await treasuryContract.initialize(
  "startup-ops",
  "Startup Operations",
  founderAddress,
  [founder, cfo, cto],
  2
);

// 2. Deposit funds
await tokenContract.approve(
  treasuryContract.address,
  1_000_000_0000000n
);

await treasuryContract.deposit(
  "startup-ops",
  nativeToken,
  1_000_000_0000000n,
  founderAddress
);
```

### Complete Proposal Flow

```typescript
// 1. Create transfer proposal
await treasuryContract.createProposal(
  "startup-ops",
  "pay-vendor-1",
  founderAddress,
  {
    Transfer: {
      recipient: vendorAddress,
      asset: nativeToken,
      amount: 50_000_0000000n
    }
  },
  currentLedger + 100000
);

// 2. Vote (founder)
await treasuryContract.vote(
  "pay-vendor-1",
  founderAddress,
  true
);

// 3. Vote (CFO) - auto-executes
await treasuryContract.vote(
  "pay-vendor-1",
  cfoAddress,
  true
);

// Proposal automatically executed!
```

### Emergency Pause

```typescript
// 1. Create pause proposal
await treasuryContract.createProposal(
  "startup-ops",
  "emergency-pause",
  founderAddress,
  { Pause: {} },
  currentLedger + 10000
);

// 2. Gather votes quickly
await treasuryContract.vote("emergency-pause", founder, true);
await treasuryContract.vote("emergency-pause", cfo, true);

// Treasury now paused

// 3. Later, resume
await treasuryContract.createProposal(
  "startup-ops",
  "resume-ops",
  founderAddress,
  { Resume: {} },
  currentLedger + 100000
);

await treasuryContract.vote("resume-ops", founder, true);
await treasuryContract.vote("resume-ops", cfo, true);
```

## Testing

Run contract tests:
```bash
cd contracts
cargo test
```

Run integration tests:
```bash
cd tests
cargo test --test integration_tests
```

Run security tests:
```bash
cd tests
cargo test --test security_tests
```

## Security Considerations

1. **Always verify signer addresses** before treasury creation
2. **Use appropriate thresholds** (2-of-3, 3-of-5, etc.)
3. **Set reasonable expiration times** for proposals
4. **Monitor events** for unauthorized activity
5. **Test pause mechanism** before production use
6. **Audit all proposal payloads** before voting
7. **Use hardware wallets** for signer keys in production
8. **Implement monitoring** for contract interactions

## Resources

- [Soroban Documentation](https://soroban.stellar.org)
- [Stellar SDK Documentation](https://stellar.github.io/js-stellar-sdk/)
- [Contract Source Code](../contracts/)
- [Integration Tests](../tests/)
