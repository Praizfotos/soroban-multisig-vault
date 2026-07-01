#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};
use types::AttestationInput;

fn create_contract() -> (Env, Address, AttestationRegistryClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, AttestationRegistry);
    let client = AttestationRegistryClient::new(&env, &contract_id);

    let admin = Address::generate(&env);

    (env, admin, client)
}

#[test]
fn test_initialize() {
    let (env, admin, client) = create_contract();

    let result = client.initialize(&admin);
    assert!(result.is_ok());

    let info = client.get_info();
    assert_eq!(info.admin, admin);
    assert_eq!(info.total_attestations, 0);
}

#[test]
fn test_initialize_twice_fails() {
    let (env, admin, client) = create_contract();

    client.initialize(&admin).unwrap();

    let result = client.initialize(&admin);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn test_add_trusted_issuer() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);

    let result = client.add_trusted_issuer(&issuer);
    assert!(result.is_ok());

    assert!(client.is_trusted_issuer(&issuer));
}

#[test]
fn test_add_trusted_issuer_twice_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let result = client.add_trusted_issuer(&issuer);
    assert_eq!(result, Err(Ok(Error::IssuerAlreadyTrusted)));
}

#[test]
fn test_remove_trusted_issuer() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();
    assert!(client.is_trusted_issuer(&issuer));

    let result = client.remove_trusted_issuer(&issuer);
    assert!(result.is_ok());

    assert!(!client.is_trusted_issuer(&issuer));
}

#[test]
fn test_remove_nonexistent_issuer_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);

    let result = client.remove_trusted_issuer(&issuer);
    assert_eq!(result, Err(Ok(Error::IssuerNotFound)));
}

#[test]
fn test_issue_attestation() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2 verification completed"),
    };

    let result = client.issue_attestation(&issuer, &input);
    assert!(result.is_ok());

    let attestation_id = result.unwrap();
    assert_eq!(attestation_id, 1);

    let attestation = client.get_attestation(&attestation_id);
    assert!(attestation.is_some());

    let attestation = attestation.unwrap();
    assert_eq!(attestation.id, 1);
    assert_eq!(attestation.issuer, issuer);
    assert_eq!(attestation.subject, subject);
    assert_eq!(
        attestation.attestation_type,
        String::from_str(&env, "kyc_verified")
    );
    assert!(!attestation.revoked);
}

#[test]
fn test_issue_attestation_not_trusted_issuer_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2 verification completed"),
    };

    let result = client.issue_attestation(&issuer, &input);
    assert_eq!(result, Err(Ok(Error::NotTrustedIssuer)));
}

#[test]
fn test_issue_attestation_invalid_input_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, ""),
        data: String::from_str(&env, "data"),
    };

    let result = client.issue_attestation(&issuer, &input);
    assert_eq!(result, Err(Ok(Error::InvalidInput)));
}

#[test]
fn test_revoke_attestation() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2 verification completed"),
    };

    let attestation_id = client.issue_attestation(&issuer, &input).unwrap();

    let result = client.revoke_attestation(&issuer, &attestation_id);
    assert!(result.is_ok());

    let attestation = client.get_attestation(&attestation_id).unwrap();
    assert!(attestation.revoked);
}

#[test]
fn test_revoke_nonexistent_attestation_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    client.add_trusted_issuer(&issuer).unwrap();

    let result = client.revoke_attestation(&issuer, &999);
    assert_eq!(result, Err(Ok(Error::AttestationNotFound)));
}

#[test]
fn test_revoke_others_attestation_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer1 = Address::generate(&env);
    let issuer2 = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer1).unwrap();
    client.add_trusted_issuer(&issuer2).unwrap();

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2 verification completed"),
    };

    let attestation_id = client.issue_attestation(&issuer1, &input).unwrap();

    let result = client.revoke_attestation(&issuer2, &attestation_id);
    assert_eq!(result, Err(Ok(Error::CannotRevokeOthersAttestation)));
}

#[test]
fn test_revoke_already_revoked_fails() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2 verification completed"),
    };

    let attestation_id = client.issue_attestation(&issuer, &input).unwrap();

    client.revoke_attestation(&issuer, &attestation_id).unwrap();

    let result = client.revoke_attestation(&issuer, &attestation_id);
    assert_eq!(result, Err(Ok(Error::AttestationAlreadyRevoked)));
}

#[test]
fn test_get_attestations_by_subject() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input1 = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2"),
    };

    let input2 = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "contributor"),
        data: String::from_str(&env, "Active contributor"),
    };

    client.issue_attestation(&issuer, &input1).unwrap();
    client.issue_attestation(&issuer, &input2).unwrap();

    let attestations = client.get_attestations_by_subject(&subject);
    assert_eq!(attestations.len(), 2);
}

#[test]
fn test_get_attestations_by_issuer() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject1 = Address::generate(&env);
    let subject2 = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input1 = AttestationInput {
        subject: subject1.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2"),
    };

    let input2 = AttestationInput {
        subject: subject2.clone(),
        attestation_type: String::from_str(&env, "contributor"),
        data: String::from_str(&env, "Active contributor"),
    };

    client.issue_attestation(&issuer, &input1).unwrap();
    client.issue_attestation(&issuer, &input2).unwrap();

    let attestations = client.get_attestations_by_issuer(&issuer);
    assert_eq!(attestations.len(), 2);
}

#[test]
fn test_transfer_admin() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let new_admin = Address::generate(&env);

    let result = client.transfer_admin(&new_admin);
    assert!(result.is_ok());

    let info = client.get_info();
    assert_eq!(info.admin, new_admin);
}

#[test]
fn test_multiple_attestations_increment_counter() {
    let (env, admin, client) = create_contract();
    client.initialize(&admin).unwrap();

    let issuer = Address::generate(&env);
    let subject = Address::generate(&env);

    client.add_trusted_issuer(&issuer).unwrap();

    let input = AttestationInput {
        subject: subject.clone(),
        attestation_type: String::from_str(&env, "kyc_verified"),
        data: String::from_str(&env, "Level 2"),
    };

    let id1 = client.issue_attestation(&issuer, &input).unwrap();
    let id2 = client.issue_attestation(&issuer, &input).unwrap();
    let id3 = client.issue_attestation(&issuer, &input).unwrap();

    assert_eq!(id1, 1);
    assert_eq!(id2, 2);
    assert_eq!(id3, 3);

    let info = client.get_info();
    assert_eq!(info.total_attestations, 3);
}
