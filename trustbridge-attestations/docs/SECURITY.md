# Security Model

## Overview

TrustBridge Attestations implements multiple security layers to ensure the integrity, authenticity, and reliability of on-chain attestations.

## Trust Model

### Core Assumptions

1. **Admin Trust**: The contract admin is a trusted entity responsible for issuer management
2. **Issuer Trust**: Trusted issuers are vetted entities authorized to issue attestations
3. **Stellar Security**: The underlying Stellar network is secure and reliable
4. **Wallet Security**: Users maintain secure custody of their private keys

### Trust Boundaries

- **Admin → Contract**: Admin has full control over trusted issuer list
- **Issuer → Attestations**: Issuers have full control over their attestations
- **Subject → Nothing**: Subjects cannot modify attestations about themselves
- **Public → Read-Only**: Anyone can query but not modify data

## Authorization Controls

### Admin Operations

Protected by `admin.require_auth()`:

```rust
- initialize(admin)
- add_trusted_issuer(issuer)
- remove_trusted_issuer(issuer)
- transfer_admin(new_admin)
```

**Attack Vector**: Compromised admin key
**Mitigation**: Use multi-sig wallet or hardware wallet for admin operations

### Issuer Operations

Protected by `issuer.require_auth()` + issuer trust check:

```rust
- issue_attestation(issuer, input)
- revoke_attestation(issuer, id)
```

**Attack Vector**: Compromised issuer key
**Mitigation**: 
- Issuer can revoke all their attestations
- Admin can remove compromised issuer from trust list
- Monitor for suspicious attestation patterns

### Additional Checks

- **Ownership Check**: Only original issuer can revoke their attestation
- **Status Check**: Cannot revoke already-revoked attestation
- **Input Validation**: Empty type or data strings rejected

## Threat Analysis

### 1. Unauthorized Attestation Issuance

**Threat**: Attacker issues fake attestations

**Mitigations**:
- Trusted issuer whitelist enforced on-chain
- Authorization required for all issuance
- Only admin can add trusted issuers

**Residual Risk**: Compromised issuer key (see mitigation above)

### 2. Attestation Tampering

**Threat**: Attacker modifies existing attestations

**Mitigations**:
- Attestations are immutable (except revoked flag)
- Revocation only by original issuer
- All data stored on immutable blockchain

**Residual Risk**: None (blockchain immutability)

### 3. Replay Attacks

**Threat**: Attacker resubmits valid transactions

**Mitigations**:
- Stellar transaction sequence numbers prevent replay
- Each transaction modifies contract state
- Idempotency checks (e.g., cannot add issuer twice)

**Residual Risk**: None (blockchain-level protection)

### 4. Denial of Service

**Threat**: Attacker floods contract with transactions

**Mitigations**:
- Stellar transaction fees make spam expensive
- No unbounded loops in contract code
- All operations have constant or linear complexity
- Storage is persistent (not limited by transaction)

**Residual Risk**: High-cost DoS possible but economically irrational

### 5. Admin Key Compromise

**Threat**: Attacker gains admin private key

**Impact**:
- Can add malicious trusted issuers
- Can remove legitimate issuers
- Cannot modify existing attestations

**Mitigations**:
- Use multi-signature wallet for admin
- Use hardware wallet for admin key
- Monitor admin operations off-chain
- Consider time-locks on admin operations
- Implement admin transfer as emergency measure

**Residual Risk**: Medium (depends on admin key security)

### 6. Issuer Key Compromise

**Threat**: Attacker gains issuer private key

**Impact**:
- Can issue fake attestations under issuer's name
- Can revoke issuer's legitimate attestations

**Mitigations**:
- Issuer can self-revoke all attestations
- Admin can remove issuer from trust list
- Monitor issuer patterns off-chain
- Implement alerting for unusual activity

**Residual Risk**: Medium until detected and mitigated

### 7. Front-Running

**Threat**: Attacker observes pending transaction and submits competing transaction

**Impact**: Minimal (operations are deterministic)

**Mitigations**:
- Attestation IDs are sequential (no race condition value)
- Operations don't depend on external state
- No economic incentive for front-running

**Residual Risk**: Low (limited attack value)

### 8. Smart Contract Bugs

**Threat**: Vulnerabilities in contract code

**Mitigations**:
- Comprehensive unit tests (100% coverage goal)
- Static analysis with clippy
- Code auditing before mainnet deployment
- Simple, auditable contract logic
- No external contract calls (no reentrancy risk)
- No complex math (no overflow/underflow risk)

**Residual Risk**: Low (simple contract with tests)

### 9. Oracle Attacks

**Threat**: Attacker manipulates external data sources

**Impact**: Not applicable (no oracles used)

**Residual Risk**: None

### 10. Governance Capture

**Threat**: Attacker influences who becomes trusted issuer

**Mitigations**:
- Clear admin selection process (off-chain)
- Consider DAO governance for admin role
- Transparent issuer vetting process
- Public issuer registry

**Residual Risk**: Depends on governance model

## Security Best Practices

### For Administrators

1. **Key Management**
   - Use hardware wallet or multi-sig
   - Never share or expose admin private key
   - Regularly rotate keys if possible
   - Have key recovery plan

2. **Issuer Vetting**
   - Establish clear criteria for trusted issuers
   - Document vetting process
   - Perform background checks
   - Monitor issuer behavior

3. **Incident Response**
   - Have plan for compromised issuer
   - Can quickly remove malicious issuers
   - Communicate incidents to community
   - Maintain audit logs

### For Trusted Issuers

1. **Key Security**
   - Use hardware wallet for issuer key
   - Never share private key
   - Limit key exposure
   - Use separate key per environment (testnet/mainnet)

2. **Operational Security**
   - Validate subject addresses before issuance
   - Document attestation criteria
   - Monitor for suspicious activity
   - Have revocation procedures

3. **Data Quality**
   - Ensure accurate attestation data
   - Follow data schemas/standards
   - Regularly review issued attestations
   - Revoke outdated/invalid attestations

### For Developers

1. **Integration Security**
   - Validate all user inputs
   - Use transaction simulation before signing
   - Handle errors gracefully
   - Don't expose private keys in frontend

2. **Frontend Security**
   - Validate Freighter responses
   - Check network matches expected
   - Verify contract ID is correct
   - Use HTTPS for all connections

3. **Testing**
   - Test error conditions
   - Test authorization failures
   - Test edge cases
   - Test with malicious inputs

## Audit Trail

All operations emit events for monitoring:

```rust
initialized
trusted_issuer_added
trusted_issuer_removed
attestation_issued
attestation_revoked
admin_transferred
```

### Recommended Monitoring

- Alert on new trusted issuers
- Alert on admin transfers
- Track attestation volume per issuer
- Detect unusual patterns
- Log all admin operations

## Upgrade Path

Current contract is **not upgradeable** by design:

**Pros**:
- Immutable logic = predictable behavior
- No admin backdoor for upgrades
- Simpler security model

**Cons**:
- Cannot fix bugs without redeployment
- Cannot add features without migration

**Migration Strategy**:
1. Deploy new contract version
2. Admin adds note about new contract
3. Update frontend to new contract
4. Old attestations remain on old contract
5. Consider bridge contract for unified view

## Incident Response Plan

### Compromised Issuer

1. **Detect**: Monitor flags unusual attestation activity
2. **Verify**: Admin investigates and confirms compromise
3. **Remove**: Admin calls `remove_trusted_issuer`
4. **Communicate**: Notify community of compromised attestations
5. **Mark**: Off-chain system marks affected attestations
6. **Recover**: Issuer rotates keys and requests re-addition

### Compromised Admin

1. **Emergency**: New admin transfer requires community coordination
2. **Options**:
   - If admin has multi-sig: Other signers transfer admin
   - If single-sig: May require new contract deployment
3. **Prevention**: Use multi-sig from start

### Smart Contract Bug

1. **Disclosure**: Report to team via security channel
2. **Assessment**: Evaluate severity and exploitability
3. **Fix**: Develop and test patch
4. **Deploy**: Deploy new contract version
5. **Migrate**: Update all integrations
6. **Notify**: Inform users and stakeholders

## Security Roadmap

### Short Term

- [ ] External security audit
- [ ] Formal verification of core functions
- [ ] Expanded test suite with fuzzing
- [ ] Monitoring dashboard for admin

### Medium Term

- [ ] Multi-sig admin implementation
- [ ] Time-locks on sensitive operations
- [ ] Delegated issuance capabilities
- [ ] Privacy-preserving attestations

### Long Term

- [ ] DAO governance for admin role
- [ ] Cross-chain attestation verification
- [ ] Zero-knowledge proof integration
- [ ] Decentralized issuer reputation

## Conclusion

TrustBridge Attestations implements a defense-in-depth security model appropriate for its trust assumptions. The primary security considerations are:

1. **Admin key security** (highest priority)
2. **Issuer key security** (high priority)
3. **Smart contract correctness** (high priority)
4. **Operational monitoring** (medium priority)

By following the security best practices outlined in this document, administrators, issuers, and developers can maintain a secure attestation system.

---

**Security Contact**: Report vulnerabilities to [SECURITY_CONTACT_EMAIL]

**Last Updated**: 2026-06-30
