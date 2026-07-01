#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GovernanceConfig {
    pub treasury_contract: Address,
    pub voting_period: u32,
    pub execution_delay: u32,
    pub quorum_threshold: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VoteRecord {
    pub proposal_id: String,
    pub voter: Address,
    pub vote_type: VoteType,
    pub voting_power: u32,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VoteType {
    Approve,
    Reject,
    Abstain,
}

#[contract]
pub struct GovernanceContract;

#[contractimpl]
impl GovernanceContract {
    /// Initialize governance configuration
    pub fn initialize(
        env: Env,
        treasury_contract: Address,
        voting_period: u32,
        execution_delay: u32,
        quorum_threshold: u32,
    ) {
        let config = GovernanceConfig {
            treasury_contract,
            voting_period,
            execution_delay,
            quorum_threshold,
        };
        
        env.storage().instance().set(&String::from_str(&env, "CONFIG"), &config);
    }

    /// Get governance configuration
    pub fn get_config(env: Env) -> GovernanceConfig {
        env.storage()
            .instance()
            .get(&String::from_str(&env, "CONFIG"))
            .unwrap()
    }

    /// Record a vote
    pub fn record_vote(
        env: Env,
        proposal_id: String,
        voter: Address,
        vote_type: VoteType,
        voting_power: u32,
    ) {
        voter.require_auth();

        let vote = VoteRecord {
            proposal_id: proposal_id.clone(),
            voter: voter.clone(),
            vote_type,
            voting_power,
            timestamp: env.ledger().timestamp(),
        };

        let key = (String::from_str(&env, "VOTE"), proposal_id, voter);
        env.storage().persistent().set(&key, &vote);
    }

    /// Get vote record
    pub fn get_vote(env: Env, proposal_id: String, voter: Address) -> Option<VoteRecord> {
        let key = (String::from_str(&env, "VOTE"), proposal_id, voter);
        env.storage().persistent().get(&key)
    }

    /// Calculate total votes for a proposal
    pub fn get_vote_counts(
        env: Env,
        proposal_id: String,
        voters: Vec<Address>,
    ) -> (u32, u32, u32) {
        let mut approve_count = 0u32;
        let mut reject_count = 0u32;
        let mut abstain_count = 0u32;

        for voter in voters.iter() {
            if let Some(vote) = Self::get_vote(env.clone(), proposal_id.clone(), voter) {
                match vote.vote_type {
                    VoteType::Approve => approve_count += vote.voting_power,
                    VoteType::Reject => reject_count += vote.voting_power,
                    VoteType::Abstain => abstain_count += vote.voting_power,
                }
            }
        }

        (approve_count, reject_count, abstain_count)
    }

    /// Check if quorum is reached
    pub fn check_quorum(
        env: Env,
        approve_count: u32,
        reject_count: u32,
        abstain_count: u32,
        total_voting_power: u32,
    ) -> bool {
        let config = Self::get_config(env);
        let total_votes = approve_count + reject_count + abstain_count;
        let quorum_required = (total_voting_power * config.quorum_threshold) / 100;
        
        total_votes >= quorum_required
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize_governance() {
        let env = Env::default();
        let treasury = Address::generate(&env);
        
        let client = GovernanceContractClient::new(&env, &env.register_contract(None, GovernanceContract));
        
        client.initialize(&treasury, &86400, &3600, &50);
        
        let config = client.get_config();
        assert_eq!(config.voting_period, 86400);
        assert_eq!(config.execution_delay, 3600);
        assert_eq!(config.quorum_threshold, 50);
    }

    #[test]
    fn test_record_and_get_vote() {
        let env = Env::default();
        env.mock_all_auths();
        
        let treasury = Address::generate(&env);
        let voter = Address::generate(&env);
        
        let client = GovernanceContractClient::new(&env, &env.register_contract(None, GovernanceContract));
        
        client.initialize(&treasury, &86400, &3600, &50);
        
        let proposal_id = String::from_str(&env, "prop-1");
        client.record_vote(&proposal_id, &voter, &VoteType::Approve, &1);
        
        let vote = client.get_vote(&proposal_id, &voter).unwrap();
        assert_eq!(vote.vote_type, VoteType::Approve);
        assert_eq!(vote.voting_power, 1);
    }

    #[test]
    fn test_vote_counts() {
        let env = Env::default();
        env.mock_all_auths();
        
        let treasury = Address::generate(&env);
        let voter1 = Address::generate(&env);
        let voter2 = Address::generate(&env);
        let voter3 = Address::generate(&env);
        
        let client = GovernanceContractClient::new(&env, &env.register_contract(None, GovernanceContract));
        
        client.initialize(&treasury, &86400, &3600, &50);
        
        let proposal_id = String::from_str(&env, "prop-1");
        
        client.record_vote(&proposal_id, &voter1, &VoteType::Approve, &1);
        client.record_vote(&proposal_id, &voter2, &VoteType::Approve, &1);
        client.record_vote(&proposal_id, &voter3, &VoteType::Reject, &1);
        
        let mut voters = Vec::new(&env);
        voters.push_back(voter1);
        voters.push_back(voter2);
        voters.push_back(voter3);
        
        let (approve, reject, abstain) = client.get_vote_counts(&proposal_id, &voters);
        assert_eq!(approve, 2);
        assert_eq!(reject, 1);
        assert_eq!(abstain, 0);
    }

    #[test]
    fn test_quorum_check() {
        let env = Env::default();
        let treasury = Address::generate(&env);
        
        let client = GovernanceContractClient::new(&env, &env.register_contract(None, GovernanceContract));
        
        // 50% quorum threshold
        client.initialize(&treasury, &86400, &3600, &50);
        
        // Test with 60% participation - should pass
        let quorum_reached = client.check_quorum(&4, &2, &0, &10);
        assert_eq!(quorum_reached, true);
        
        // Test with 40% participation - should fail
        let quorum_not_reached = client.check_quorum(&2, &1, &1, &10);
        assert_eq!(quorum_not_reached, false);
    }
}
