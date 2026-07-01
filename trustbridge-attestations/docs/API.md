# API Reference

Complete reference for the TrustBridge Attestations smart contract interface.

## Contract Functions

### Initialize

Initialize the contract with an admin address.

```rust
pub fn initialize(env: Env, admin: Address) -> Result<(), Error>
```

**Parameters:**
- `admin`: The address that will become the contract administrator

**Returns:**
- `Ok(())` on success
- `Err(Error::AlreadyInitialized)` if contract is already initialized

**Authorization:** Requires `admin` signature

**Events:**
- Emits `("initialized", admin)`

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  -- initialize \
  --admin <ADMIN_ADDRESS>
```

---

### Add Trusted Issuer

Add an address to the trusted issuer whitelist.

```rust
pub fn add_trusted_issuer(env: Env, issuer: Address) -> Result<(), Error>
```

**Parameters:**
- `issuer`: The address to add as a trusted issuer

**Returns:**
- `Ok(())` on success
- `Err(Error::Unauthorized)` if caller is not admin
- `Err(Error::IssuerAlreadyTrusted)` if issuer is already trusted

**Authorization:** Admin only

**Events:**
- Emits `("trusted_issuer_added", issuer)`

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  -- add_trusted_issuer \
  --issuer <ISSUER_ADDRESS>
```

---

### Remove Trusted Issuer

Remove an address from the trusted issuer whitelist.

```rust
pub fn remove_trusted_issuer(env: Env, issuer: Address) -> Result<(), Error>
```

**Parameters:**
- `issuer`: The address to remove from trusted issuers

**Returns:**
- `Ok(())` on success
- `Err(Error::Unauthorized)` if caller is not admin
- `Err(Error::IssuerNotFound)` if issuer is not trusted

**Authorization:** Admin only

**Events:**
- Emits `("trusted_issuer_removed", issuer)`

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  -- remove_trusted_issuer \
  --issuer <ISSUER_ADDRESS>
```

---

### Is Trusted Issuer

Check if an address is a trusted issuer.

```rust
pub fn is_trusted_issuer(env: Env, issuer: Address) -> bool
```

**Parameters:**
- `issuer`: The address to check

**Returns:**
- `true` if the address is a trusted issuer
- `false` otherwise

**Authorization:** None (public query)

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  -- is_trusted_issuer \
  --issuer <ISSUER_ADDRESS>
```

---

### Issue Attestation

Issue a new attestation.

```rust
pub fn issue_attestation(
    env: Env,
    issuer: Address,
    input: AttestationInput
) -> Result<u64, Error>
```

**Parameters:**
- `issuer`: The address of the issuer (must be trusted)
- `input`: Attestation input data
  - `subject`: Address the attestation is about
  - `attestation_type`: Type/category of attestation (non-empty string)
  - `data`: Attestation content (non-empty string)

**Returns:**
- `Ok(id)` with the new attestation ID on success
- `Err(Error::NotTrustedIssuer)` if issuer is not trusted
- `Err(Error::InvalidInput)` if type or data is empty

**Authorization:** Requires `issuer` signature, issuer must be trusted

**Events:**
- Emits `("attestation_issued", (id, issuer, subject))`

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source issuer \
  -- issue_attestation \
  --issuer <ISSUER_ADDRESS> \
  --input '{"subject":"<SUBJECT_ADDRESS>","attestation_type":"kyc_verified","data":"Level 2 verification completed"}'
```

---

### Revoke Attestation

Revoke an existing attestation.

```rust
pub fn revoke_attestation(env: Env, issuer: Address, id: u64) -> Result<(), Error>
```

**Parameters:**
- `issuer`: The address of the issuer
- `id`: The attestation ID to revoke

**Returns:**
- `Ok(())` on success
- `Err(Error::AttestationNotFound)` if attestation doesn't exist
- `Err(Error::CannotRevokeOthersAttestation)` if caller is not the original issuer
- `Err(Error::AttestationAlreadyRevoked)` if already revoked

**Authorization:** Requires `issuer` signature, must be original issuer

**Events:**
- Emits `("attestation_revoked", (id, issuer))`

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source issuer \
  -- revoke_attestation \
  --issuer <ISSUER_ADDRESS> \
  --id 1
```

---

### Get Attestation

Retrieve a specific attestation by ID.

```rust
pub fn get_attestation(env: Env, id: u64) -> Option<Attestation>
```

**Parameters:**
- `id`: The attestation ID

**Returns:**
- `Some(attestation)` if found
- `None` if not found

**Authorization:** None (public query)

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  -- get_attestation \
  --id 1
```

---

### Get Attestations By Subject

Retrieve all attestations for a subject.

```rust
pub fn get_attestations_by_subject(env: Env, subject: Address) -> Vec<Attestation>
```

**Parameters:**
- `subject`: The address to query

**Returns:**
- Vector of all attestations for the subject (may be empty)

**Authorization:** None (public query)

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  -- get_attestations_by_subject \
  --subject <SUBJECT_ADDRESS>
```

---

### Get Attestations By Issuer

Retrieve all attestations issued by an issuer.

```rust
pub fn get_attestations_by_issuer(env: Env, issuer: Address) -> Vec<Attestation>
```

**Parameters:**
- `issuer`: The address to query

**Returns:**
- Vector of all attestations issued by the issuer (may be empty)

**Authorization:** None (public query)

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  -- get_attestations_by_issuer \
  --issuer <ISSUER_ADDRESS>
```

---

### Get Info

Get contract information and statistics.

```rust
pub fn get_info(env: Env) -> ContractInfo
```

**Returns:**
- `ContractInfo` struct containing:
  - `admin`: Current admin address
  - `total_attestations`: Total number of attestations issued
  - `total_trusted_issuers`: Reserved field (currently 0)

**Authorization:** None (public query)

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  -- get_info
```

---

### Transfer Admin

Transfer admin rights to a new address.

```rust
pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error>
```

**Parameters:**
- `new_admin`: The new admin address

**Returns:**
- `Ok(())` on success
- `Err(Error::Unauthorized)` if caller is not current admin

**Authorization:** Admin only

**Events:**
- Emits `("admin_transferred", (old_admin, new_admin))`

**Example:**
```bash
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source admin \
  -- transfer_admin \
  --new_admin <NEW_ADMIN_ADDRESS>
```

---

## Data Types

### Attestation

```rust
pub struct Attestation {
    pub id: u64,
    pub issuer: Address,
    pub subject: Address,
    pub attestation_type: String,
    pub data: String,
    pub timestamp: u64,
    pub revoked: bool,
}
```

**Fields:**
- `id`: Unique identifier (sequential)
- `issuer`: Who issued the attestation
- `subject`: Who the attestation is about
- `attestation_type`: Category or type of attestation
- `data`: The attestation content/details
- `timestamp`: Unix timestamp when issued (ledger time)
- `revoked`: Whether the attestation has been revoked

### AttestationInput

```rust
pub struct AttestationInput {
    pub subject: Address,
    pub attestation_type: String,
    pub data: String,
}
```

**Fields:**
- `subject`: Address the attestation is about
- `attestation_type`: Type/category (must be non-empty)
- `data`: Attestation content (must be non-empty)

### ContractInfo

```rust
pub struct ContractInfo {
    pub admin: Address,
    pub total_attestations: u64,
    pub total_trusted_issuers: u64,
}
```

**Fields:**
- `admin`: Current admin address
- `total_attestations`: Total attestations issued
- `total_trusted_issuers`: Reserved (currently not tracked)

---

## Error Codes

```rust
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    NotTrustedIssuer = 4,
    IssuerAlreadyTrusted = 5,
    IssuerNotFound = 6,
    AttestationNotFound = 7,
    AttestationAlreadyRevoked = 8,
    InvalidInput = 9,
    CannotRevokeOthersAttestation = 10,
}
```

---

## Events

All contract operations emit events for off-chain indexing and monitoring.

### Event: initialized

Emitted when contract is initialized.

**Data:** `admin: Address`

### Event: trusted_issuer_added

Emitted when a trusted issuer is added.

**Data:** `issuer: Address`

### Event: trusted_issuer_removed

Emitted when a trusted issuer is removed.

**Data:** `issuer: Address`

### Event: attestation_issued

Emitted when an attestation is issued.

**Data:** `(id: u64, issuer: Address, subject: Address)`

### Event: attestation_revoked

Emitted when an attestation is revoked.

**Data:** `(id: u64, issuer: Address)`

### Event: admin_transferred

Emitted when admin rights are transferred.

**Data:** `(old_admin: Address, new_admin: Address)`

---

## Usage Patterns

### Complete Workflow Example

```bash
# 1. Initialize contract
soroban contract invoke --id $CONTRACT_ID --source admin \
  -- initialize --admin $ADMIN_ADDR

# 2. Add trusted issuer
soroban contract invoke --id $CONTRACT_ID --source admin \
  -- add_trusted_issuer --issuer $ISSUER_ADDR

# 3. Issue attestation
soroban contract invoke --id $CONTRACT_ID --source issuer \
  -- issue_attestation \
  --issuer $ISSUER_ADDR \
  --input '{"subject":"'$SUBJECT_ADDR'","attestation_type":"contributor","data":"Active contributor since 2024"}'

# 4. Query attestations
soroban contract invoke --id $CONTRACT_ID \
  -- get_attestations_by_subject --subject $SUBJECT_ADDR

# 5. Revoke attestation (if needed)
soroban contract invoke --id $CONTRACT_ID --source issuer \
  -- revoke_attestation --issuer $ISSUER_ADDR --id 1
```

---

## Rate Limits and Costs

### Transaction Fees

All operations incur Stellar transaction fees:
- Base fee: 100 stroops minimum
- Resource fees: Based on contract execution

### No Built-in Rate Limits

The contract does not enforce rate limits. Consider implementing off-chain rate limiting in production applications.

---

## Best Practices

1. **Validate inputs** before submitting transactions
2. **Simulate transactions** before signing in production
3. **Monitor events** for all operations
4. **Cache query results** to reduce RPC calls
5. **Handle errors gracefully** in client applications
6. **Use meaningful attestation types** for consistency
7. **Document your attestation schema** for interoperability

---

For more information, see:
- [Architecture Documentation](./ARCHITECTURE.md)
- [Security Model](./SECURITY.md)
- [README](../README.md)
