#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::{Address as _, Ledger, LedgerInfo}, Address, Env, String, Vec};

fn create_test_treasury(env: &Env) -> (Address, Address, Address, String) {
    let creator = Address::generate(env);
    let signer1 = Address::generate(env);
    let signer2 = Address::generate(env);
    
    let treasury_id = String::from_str(env, "test-treasury");
    let name = String::from_str(env, "Test Treasury");
    
    let mut signers = Vec::new(env);
    signers.push_back(creator.clone());
    signers.push_back(signer1.clone());
    signers.push_back(signer2.clone());
    
    let client = TreasuryContractClient::new(env, &env.register_contract(None, TreasuryContract));
    
    client.initialize(&treasury_id, &name, &creator, &signers, &2);
    
    (creator, signer1, signer2, treasury_id)
}

#[test]
fn test_initialize_treasury() {
    let env = Env::default();
    env.mock_all_auths();
    
    let creator = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    
    let treasury_id = String::from_str(&env, "test-treasury");
    let name = String::from_str(&env, "Test Treasury");
    
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer1);
    signers.push_back(signer2);
    
    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));
    
    client.initialize(&treasury_id, &name, &creator, &signers, &2);
    
    let treasury = client.get_treasury(&treasury_id);
    assert_eq!(treasury.id, treasury_id);
    assert_eq!(treasury.name, name);
    assert_eq!(treasury.threshold, 2);
    assert_eq!(treasury.signers.len(), 3);
    assert_eq!(treasury.paused, false);
}

#[test]
#[should_panic(expected = "InvalidThreshold")]
fn test_invalid_threshold() {
    let env = Env::default();
    env.mock_all_auths();
    
    let creator = Address::generate(&env);
    let signer1 = Address::generate(&env);
    
    let treasury_id = String::from_str(&env, "test-treasury");
    let name = String::from_str(&env, "Test Treasury");
    
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer1);
    
    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));
    
    // Threshold 3 with only 2 signers should fail
    client.initialize(&treasury_id, &name, &creator, &signers, &3);
}

#[test]
fn test_create_and_approve_proposal() {
    let env = Env::default();
    env.mock_all_auths();
    
    env.ledger().set(LedgerInfo {
        timestamp: 1000,
        protocol_version: 20,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });
    
    let (creator, signer1, signer2, treasury_id) = create_test_treasury(&env);
    
    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));
    
    // Create a pause proposal
    let proposal_id = String::from_str(&env, "proposal-1");
    let expiration = 200u32;
    
    client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::Pause,
        &expiration,
    );
    
    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.status, ProposalStatus::Pending);
    
    // First approval
    client.vote(&proposal_id, &creator, &true);
    
    // Second approval - should auto-execute
    client.vote(&proposal_id, &signer1, &true);
    
    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.status, ProposalStatus::Approved);
    assert_eq!(proposal.executed, true);
    
    // Verify treasury is paused
    let treasury = client.get_treasury(&treasury_id);
    assert_eq!(treasury.paused, true);
}

#[test]
#[should_panic(expected = "AlreadyVoted")]
fn test_double_voting() {
    let env = Env::default();
    env.mock_all_auths();
    
    env.ledger().set(LedgerInfo {
        timestamp: 1000,
        protocol_version: 20,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });
    
    let (creator, _, _, treasury_id) = create_test_treasury(&env);
    
    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));
    
    let proposal_id = String::from_str(&env, "proposal-1");
    client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::Pause,
        &200,
    );
    
    // First vote
    client.vote(&proposal_id, &creator, &true);
    
    // Second vote from same signer should fail
    client.vote(&proposal_id, &creator, &true);
}

#[test]
fn test_proposal_rejection() {
    let env = Env::default();
    env.mock_all_auths();
    
    env.ledger().set(LedgerInfo {
        timestamp: 1000,
        protocol_version: 20,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });
    
    let (creator, signer1, signer2, treasury_id) = create_test_treasury(&env);
    
    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));
    
    let proposal_id = String::from_str(&env, "proposal-1");
    client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::Pause,
        &200,
    );
    
    // Two rejections should reject the proposal (threshold is 2/3)
    client.vote(&proposal_id, &creator, &false);
    client.vote(&proposal_id, &signer1, &false);
    
    let proposal = client.get_proposal(&proposal_id);
    assert_eq!(proposal.status, ProposalStatus::Rejected);
    assert_eq!(proposal.executed, false);
}

#[test]
fn test_add_signer() {
    let env = Env::default();
    env.mock_all_auths();
    
    env.ledger().set(LedgerInfo {
        timestamp: 1000,
        protocol_version: 20,
        sequence_number: 100,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });
    
    let (creator, signer1, _, treasury_id) = create_test_treasury(&env);
    
    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));
    
    let new_signer = Address::generate(&env);
    let proposal_id = String::from_str(&env, "proposal-1");
    
    client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::AddSigner { new_signer: new_signer.clone() },
        &200,
    );
    
    // Approve and execute
    client.vote(&proposal_id, &creator, &true);
    client.vote(&proposal_id, &signer1, &true);
    
    // Verify new signer added
    let treasury = client.get_treasury(&treasury_id);
    assert_eq!(treasury.signers.len(), 4);
    assert!(treasury.signers.contains(&new_signer));
}
