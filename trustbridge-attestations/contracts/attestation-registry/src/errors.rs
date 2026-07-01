use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
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
