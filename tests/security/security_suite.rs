#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, String, Vec,
};
use treasury::{TreasuryContract, TreasuryContractClient, ProposalType};

#[test]
#[should_panic(expected = "InvalidThreshold")]
fn test_prevent_invalid_threshold_zero() {
    let env = Env::default();
    env.mock_all_auths();

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    // Threshold 0 should fail
    client.initialize(&treasury_id, &name, &creator, &signers, &0);
}

#[test]
#[should_panic(expected = "InvalidThreshold")]
fn test_prevent_invalid_threshold_too_high() {
    let env = Env::default();
    env.mock_all_auths();

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(Address::generate(&env));

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    // Threshold 5 with only 2 signers should fail
    client.initialize(&treasury_id, &name, &creator, &signers, &5);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn test_prevent_unauthorized_proposal() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    client.initialize(&treasury_id, &name, &creator, &signers, &1);

    // Non-signer trying to create proposal should fail
    let attacker = Address::generate(&env);
    let proposal_id = String::from_str(&env, "malicious");
    client.create_proposal(&treasury_id, &proposal_id, &attacker, &ProposalType::Pause, &200);
}

#[test]
#[should_panic(expected = "AlreadyVoted")]
fn test_prevent_double_voting() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(Address::generate(&env));

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    client.initialize(&treasury_id, &name, &creator, &signers, &2);

    let proposal_id = String::from_str(&env, "proposal");
    client.create_proposal(&treasury_id, &proposal_id, &creator, &ProposalType::Pause, &200);

    // First vote
    client.vote(&proposal_id, &creator, &true);

    // Second vote from same signer should fail
    client.vote(&proposal_id, &creator, &true);
}

#[test]
#[should_panic(expected = "ProposalExecuted")]
fn test_prevent_double_execution() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let signer2 = Address::generate(&env);

    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer2.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    client.initialize(&treasury_id, &name, &creator, &signers, &2);

    let proposal_id = String::from_str(&env, "proposal");
    client.create_proposal(&treasury_id, &proposal_id, &creator, &ProposalType::Pause, &200);

    // Approve and auto-execute
    client.vote(&proposal_id, &creator, &true);
    client.vote(&proposal_id, &signer2, &true);

    // Try to vote again after execution should fail
    client.vote(&proposal_id, &creator, &true);
}

#[test]
#[should_panic(expected = "TreasuryPaused")]
fn test_prevent_actions_when_paused() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let signer2 = Address::generate(&env);

    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer2.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    client.initialize(&treasury_id, &name, &creator, &signers, &2);

    // Pause treasury
    let pause_proposal = String::from_str(&env, "pause");
    client.create_proposal(&treasury_id, &pause_proposal, &creator, &ProposalType::Pause, &200);
    client.vote(&pause_proposal, &creator, &true);
    client.vote(&pause_proposal, &signer2, &true);

    // Try to create non-resume proposal when paused should fail
    let proposal_id = String::from_str(&env, "blocked");
    let new_signer = Address::generate(&env);
    client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::AddSigner {
            new_signer: new_signer.clone(),
        },
        &300,
    );
}

#[test]
#[should_panic(expected = "ProposalExpired")]
fn test_prevent_expired_proposal_execution() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let signer2 = Address::generate(&env);

    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer2.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    client.initialize(&treasury_id, &name, &creator, &signers, &2);

    let proposal_id = String::from_str(&env, "proposal");
    client.create_proposal(&treasury_id, &proposal_id, &creator, &ProposalType::Pause, &150);

    // Advance time past expiration
    env.ledger().set(LedgerInfo {
        timestamp: 2000,
        protocol_version: 20,
        sequence_number: 200,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 10,
        min_persistent_entry_ttl: 10,
        max_entry_ttl: 3110400,
    });

    // Voting on expired proposal should fail
    client.vote(&proposal_id, &creator, &true);
}

#[test]
#[should_panic(expected = "InvalidThreshold")]
fn test_prevent_invalid_threshold_after_removal() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let signer3 = Address::generate(&env);

    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer2.clone());
    signers.push_back(signer3.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    // 3-of-3 threshold
    client.initialize(&treasury_id, &name, &creator, &signers, &3);

    // Try to remove a signer (would leave 2 signers with threshold 3)
    let proposal_id = String::from_str(&env, "remove");
    client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::RemoveSigner {
            signer: signer3.clone(),
        },
        &200,
    );

    // This should fail during execution
    client.vote(&proposal_id, &creator, &true);
    client.vote(&proposal_id, &signer2, &true);
    client.vote(&proposal_id, &signer3, &true);
}

#[test]
#[should_panic(expected = "TreasuryExists")]
fn test_prevent_duplicate_treasury() {
    let env = Env::default();
    env.mock_all_auths();

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    let creator = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());

    let treasury_id = String::from_str(&env, "treasury");
    let name = String::from_str(&env, "Test");

    client.initialize(&treasury_id, &name, &creator, &signers, &1);

    // Try to create with same ID should fail
    client.initialize(&treasury_id, &name, &creator, &signers, &1);
}

#[test]
fn test_security_edge_cases() {
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

    let client = TreasuryContractClient::new(&env, &env.register_contract(None, TreasuryContract));

    // Test 1-of-1 multisig (edge case but valid)
    let creator = Address::generate(&env);
    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());

    let treasury_id = String::from_str(&env, "single");
    let name = String::from_str(&env, "Single Signer");

    client.initialize(&treasury_id, &name, &creator, &signers, &1);

    let proposal_id = String::from_str(&env, "prop1");
    client.create_proposal(&treasury_id, &proposal_id, &creator, &ProposalType::Pause, &200);

    // Single vote should execute immediately
    client.vote(&proposal_id, &creator, &true);

    let proposal = client.get_proposal(&proposal_id);
    assert!(proposal.executed);
}
