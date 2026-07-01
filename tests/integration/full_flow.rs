#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    Address, Env, String, Vec,
};
use treasury::{TreasuryContract, TreasuryContractClient, ProposalType};
use governance::{GovernanceContract, GovernanceContractClient, VoteType};
use registry::{RegistryContract, RegistryContractClient};

#[test]
fn test_complete_treasury_lifecycle() {
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

    // Deploy contracts
    let treasury_client = TreasuryContractClient::new(
        &env,
        &env.register_contract(None, TreasuryContract),
    );

    let governance_client = GovernanceContractClient::new(
        &env,
        &env.register_contract(None, GovernanceContract),
    );

    let registry_client = RegistryContractClient::new(
        &env,
        &env.register_contract(None, RegistryContract),
    );

    // Setup participants
    let creator = Address::generate(&env);
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let recipient = Address::generate(&env);

    // 1. Initialize treasury
    let treasury_id = String::from_str(&env, "dao-treasury");
    let name = String::from_str(&env, "DAO Main Treasury");

    let mut signers = Vec::new(&env);
    signers.push_back(creator.clone());
    signers.push_back(signer1.clone());
    signers.push_back(signer2.clone());

    treasury_client.initialize(&treasury_id, &name, &creator, &signers, &2);

    // 2. Register in registry
    let metadata = String::from_str(&env, r#"{"type":"DAO","purpose":"Main treasury"}"#);
    registry_client.register_treasury(
        &treasury_id,
        &name,
        &treasury_client.address,
        &creator,
        &metadata,
    );

    // 3. Initialize governance
    governance_client.initialize(&treasury_client.address, &86400, &3600, &50);

    // 4. Create pause proposal
    let proposal_id = String::from_str(&env, "proposal-pause-1");
    treasury_client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::Pause,
        &200,
    );

    // Record in registry
    registry_client.record_proposal(&treasury_id);

    // 5. Vote on proposal
    treasury_client.vote(&proposal_id, &creator, &true);
    treasury_client.vote(&proposal_id, &signer1, &true);

    // 6. Verify execution
    let proposal = treasury_client.get_proposal(&proposal_id);
    assert!(proposal.executed);

    let treasury = treasury_client.get_treasury(&treasury_id);
    assert!(treasury.paused);

    // 7. Resume treasury
    let resume_proposal_id = String::from_str(&env, "proposal-resume-1");
    treasury_client.create_proposal(
        &treasury_id,
        &resume_proposal_id,
        &creator,
        &ProposalType::Resume,
        &300,
    );

    treasury_client.vote(&resume_proposal_id, &creator, &true);
    treasury_client.vote(&resume_proposal_id, &signer1, &true);

    let treasury = treasury_client.get_treasury(&treasury_id);
    assert!(!treasury.paused);

    // 8. Add new signer
    let new_signer = Address::generate(&env);
    let add_signer_proposal = String::from_str(&env, "proposal-add-signer");

    treasury_client.create_proposal(
        &treasury_id,
        &add_signer_proposal,
        &creator,
        &ProposalType::AddSigner {
            new_signer: new_signer.clone(),
        },
        &400,
    );

    treasury_client.vote(&add_signer_proposal, &creator, &true);
    treasury_client.vote(&add_signer_proposal, &signer1, &true);

    let treasury = treasury_client.get_treasury(&treasury_id);
    assert_eq!(treasury.signers.len(), 4);
    assert!(treasury.signers.contains(&new_signer));

    // 9. Update threshold
    let threshold_proposal = String::from_str(&env, "proposal-threshold");
    treasury_client.create_proposal(
        &treasury_id,
        &threshold_proposal,
        &creator,
        &ProposalType::UpdateThreshold { new_threshold: 3 },
        &500,
    );

    treasury_client.vote(&threshold_proposal, &creator, &true);
    treasury_client.vote(&threshold_proposal, &signer1, &true);

    let treasury = treasury_client.get_treasury(&treasury_id);
    assert_eq!(treasury.threshold, 3);

    // 10. Verify registry stats
    let stats = registry_client.get_stats();
    assert_eq!(stats.total_treasuries, 1);
    assert!(stats.total_proposals > 0);
}

#[test]
fn test_multi_treasury_registry() {
    let env = Env::default();
    env.mock_all_auths();

    let registry_client = RegistryContractClient::new(
        &env,
        &env.register_contract(None, RegistryContract),
    );

    let creator1 = Address::generate(&env);
    let creator2 = Address::generate(&env);
    let contract_addr = Address::generate(&env);

    // Register multiple treasuries
    for i in 1..=5 {
        let treasury_id = String::from_str(&env, &format!("treasury-{}", i));
        let name = String::from_str(&env, &format!("Treasury {}", i));
        let metadata = String::from_str(&env, "{}");
        let creator = if i <= 3 { &creator1 } else { &creator2 };

        registry_client.register_treasury(&treasury_id, &name, &contract_addr, creator, &metadata);
    }

    // Verify total treasuries
    let all = registry_client.get_all_treasuries();
    assert_eq!(all.len(), 5);

    // Verify creator filtering
    let creator1_treasuries = registry_client.get_treasuries_by_creator(&creator1);
    assert_eq!(creator1_treasuries.len(), 3);

    let creator2_treasuries = registry_client.get_treasuries_by_creator(&creator2);
    assert_eq!(creator2_treasuries.len(), 2);
}

#[test]
fn test_governance_quorum() {
    let env = Env::default();
    env.mock_all_auths();

    let governance_client = GovernanceContractClient::new(
        &env,
        &env.register_contract(None, GovernanceContract),
    );

    let treasury_addr = Address::generate(&env);

    // 60% quorum requirement
    governance_client.initialize(&treasury_addr, &86400, &3600, &60);

    // Test various vote scenarios
    let config = governance_client.get_config();
    assert_eq!(config.quorum_threshold, 60);

    // 70% participation - should pass
    assert!(governance_client.check_quorum(&5, &2, &0, &10));

    // 50% participation - should fail
    assert!(!governance_client.check_quorum(&3, &2, &0, &10));

    // Exactly 60% - should pass
    assert!(governance_client.check_quorum(&6, &0, &0, &10));
}

#[test]
fn test_complex_voting_scenario() {
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

    let treasury_client = TreasuryContractClient::new(
        &env,
        &env.register_contract(None, TreasuryContract),
    );

    // Create 5-of-7 multisig
    let mut signers = Vec::new(&env);
    for _ in 0..7 {
        signers.push_back(Address::generate(&env));
    }

    let creator = signers.get(0).unwrap();
    let treasury_id = String::from_str(&env, "complex-treasury");
    let name = String::from_str(&env, "Complex Multisig");

    treasury_client.initialize(&treasury_id, &name, &creator, &signers, &5);

    // Create proposal
    let proposal_id = String::from_str(&env, "complex-proposal");
    treasury_client.create_proposal(
        &treasury_id,
        &proposal_id,
        &creator,
        &ProposalType::Pause,
        &200,
    );

    // Vote with exactly 5 approvals
    for i in 0..5 {
        let signer = signers.get(i).unwrap();
        treasury_client.vote(&proposal_id, &signer, &true);
    }

    // Should be approved and executed
    let proposal = treasury_client.get_proposal(&proposal_id);
    assert!(proposal.executed);
}
