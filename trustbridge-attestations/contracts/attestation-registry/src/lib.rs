#![no_std]

mod errors;
mod storage;
mod types;

#[cfg(test)]
mod test;

use errors::Error;
use soroban_sdk::{contract, contractimpl, Address, Env, Vec};
use storage::{
    add_issuer_attestation, add_subject_attestation, get_admin, get_attestation,
    get_attestation_counter, get_issuer_attestations, get_subject_attestations, has_admin,
    increment_attestation_counter, is_trusted_issuer, set_admin, set_attestation,
    set_trusted_issuer,
};
use types::{Attestation, AttestationInput, ContractInfo};

#[contract]
pub struct AttestationRegistry;

#[contractimpl]
impl AttestationRegistry {
    /// Initialize the contract with an admin
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        if has_admin(&env) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();
        set_admin(&env, &admin);

        env.events().publish(("initialized",), admin);

        Ok(())
    }

    /// Add a trusted issuer (admin only)
    pub fn add_trusted_issuer(env: Env, issuer: Address) -> Result<(), Error> {
        let admin = get_admin(&env);
        admin.require_auth();

        if is_trusted_issuer(&env, &issuer) {
            return Err(Error::IssuerAlreadyTrusted);
        }

        set_trusted_issuer(&env, &issuer, true);

        env.events().publish(("trusted_issuer_added",), issuer);

        Ok(())
    }

    /// Remove a trusted issuer (admin only)
    pub fn remove_trusted_issuer(env: Env, issuer: Address) -> Result<(), Error> {
        let admin = get_admin(&env);
        admin.require_auth();

        if !is_trusted_issuer(&env, &issuer) {
            return Err(Error::IssuerNotFound);
        }

        set_trusted_issuer(&env, &issuer, false);

        env.events().publish(("trusted_issuer_removed",), issuer);

        Ok(())
    }

    /// Check if an address is a trusted issuer
    pub fn is_trusted_issuer(env: Env, issuer: Address) -> bool {
        is_trusted_issuer(&env, &issuer)
    }

    /// Issue an attestation (trusted issuer only)
    pub fn issue_attestation(
        env: Env,
        issuer: Address,
        input: AttestationInput,
    ) -> Result<u64, Error> {
        issuer.require_auth();

        if !is_trusted_issuer(&env, &issuer) {
            return Err(Error::NotTrustedIssuer);
        }

        if input.attestation_type.len() == 0 || input.data.len() == 0 {
            return Err(Error::InvalidInput);
        }

        let id = increment_attestation_counter(&env);
        let timestamp = env.ledger().timestamp();

        let attestation = Attestation {
            id,
            issuer: issuer.clone(),
            subject: input.subject.clone(),
            attestation_type: input.attestation_type,
            data: input.data,
            timestamp,
            revoked: false,
        };

        set_attestation(&env, id, &attestation);
        add_subject_attestation(&env, &input.subject, id);
        add_issuer_attestation(&env, &issuer, id);

        env.events()
            .publish(("attestation_issued",), (id, issuer, input.subject));

        Ok(id)
    }

    /// Revoke an attestation (issuer only)
    pub fn revoke_attestation(env: Env, issuer: Address, id: u64) -> Result<(), Error> {
        issuer.require_auth();

        let mut attestation = get_attestation(&env, id).ok_or(Error::AttestationNotFound)?;

        if attestation.issuer != issuer {
            return Err(Error::CannotRevokeOthersAttestation);
        }

        if attestation.revoked {
            return Err(Error::AttestationAlreadyRevoked);
        }

        attestation.revoked = true;
        set_attestation(&env, id, &attestation);

        env.events().publish(("attestation_revoked",), (id, issuer));

        Ok(())
    }

    /// Get a specific attestation by ID
    pub fn get_attestation(env: Env, id: u64) -> Option<Attestation> {
        get_attestation(&env, id)
    }

    /// Get all attestations for a subject
    pub fn get_attestations_by_subject(env: Env, subject: Address) -> Vec<Attestation> {
        let ids = get_subject_attestations(&env, &subject);
        let mut attestations = Vec::new(&env);

        for id in ids.iter() {
            if let Some(attestation) = get_attestation(&env, id) {
                attestations.push_back(attestation);
            }
        }

        attestations
    }

    /// Get all attestations issued by an issuer
    pub fn get_attestations_by_issuer(env: Env, issuer: Address) -> Vec<Attestation> {
        let ids = get_issuer_attestations(&env, &issuer);
        let mut attestations = Vec::new(&env);

        for id in ids.iter() {
            if let Some(attestation) = get_attestation(&env, id) {
                attestations.push_back(attestation);
            }
        }

        attestations
    }

    /// Get contract information
    pub fn get_info(env: Env) -> ContractInfo {
        let admin = get_admin(&env);
        let total_attestations = get_attestation_counter(&env);

        ContractInfo {
            admin,
            total_attestations,
            total_trusted_issuers: 0, // Could be tracked separately if needed
        }
    }

    /// Transfer admin (admin only)
    pub fn transfer_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        let current_admin = get_admin(&env);
        current_admin.require_auth();

        set_admin(&env, &new_admin);

        env.events()
            .publish(("admin_transferred",), (current_admin, new_admin));

        Ok(())
    }
}
