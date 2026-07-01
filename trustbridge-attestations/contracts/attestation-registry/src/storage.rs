use crate::types::{Attestation, DataKey};
use soroban_sdk::{Address, Env, Vec};

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .unwrap_or_else(|| panic!("Admin not set"))
}

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&DataKey::Admin)
}

pub fn set_trusted_issuer(env: &Env, issuer: &Address, trusted: bool) {
    env.storage()
        .persistent()
        .set(&DataKey::TrustedIssuer(issuer.clone()), &trusted);
}

pub fn is_trusted_issuer(env: &Env, issuer: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::TrustedIssuer(issuer.clone()))
        .unwrap_or(false)
}

pub fn get_attestation_counter(env: &Env) -> u64 {
    env.storage()
        .instance()
        .get(&DataKey::AttestationCounter)
        .unwrap_or(0)
}

pub fn increment_attestation_counter(env: &Env) -> u64 {
    let counter = get_attestation_counter(env) + 1;
    env.storage()
        .instance()
        .set(&DataKey::AttestationCounter, &counter);
    counter
}

pub fn set_attestation(env: &Env, id: u64, attestation: &Attestation) {
    env.storage()
        .persistent()
        .set(&DataKey::Attestation(id), attestation);
}

pub fn get_attestation(env: &Env, id: u64) -> Option<Attestation> {
    env.storage().persistent().get(&DataKey::Attestation(id))
}

pub fn add_subject_attestation(env: &Env, subject: &Address, attestation_id: u64) {
    let key = DataKey::SubjectAttestations(subject.clone());
    let mut attestations: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));
    attestations.push_back(attestation_id);
    env.storage().persistent().set(&key, &attestations);
}

pub fn get_subject_attestations(env: &Env, subject: &Address) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::SubjectAttestations(subject.clone()))
        .unwrap_or(Vec::new(env))
}

pub fn add_issuer_attestation(env: &Env, issuer: &Address, attestation_id: u64) {
    let key = DataKey::IssuerAttestations(issuer.clone());
    let mut attestations: Vec<u64> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or(Vec::new(env));
    attestations.push_back(attestation_id);
    env.storage().persistent().set(&key, &attestations);
}

pub fn get_issuer_attestations(env: &Env, issuer: &Address) -> Vec<u64> {
    env.storage()
        .persistent()
        .get(&DataKey::IssuerAttestations(issuer.clone()))
        .unwrap_or(Vec::new(env))
}
