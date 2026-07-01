use soroban_sdk::{contracttype, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Attestation {
    pub id: u64,
    pub issuer: Address,
    pub subject: Address,
    pub attestation_type: String,
    pub data: String,
    pub timestamp: u64,
    pub revoked: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttestationInput {
    pub subject: Address,
    pub attestation_type: String,
    pub data: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    TrustedIssuer(Address),
    Attestation(u64),
    AttestationCounter,
    SubjectAttestations(Address),
    IssuerAttestations(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractInfo {
    pub admin: Address,
    pub total_attestations: u64,
    pub total_trusted_issuers: u64,
}
