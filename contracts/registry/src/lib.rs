#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TreasuryInfo {
    pub id: String,
    pub name: String,
    pub contract_address: Address,
    pub creator: Address,
    pub created_at: u64,
    pub metadata: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryStats {
    pub total_treasuries: u32,
    pub total_proposals: u32,
    pub active_treasuries: u32,
}

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    /// Register a new treasury
    pub fn register_treasury(
        env: Env,
        treasury_id: String,
        name: String,
        contract_address: Address,
        creator: Address,
        metadata: String,
    ) {
        creator.require_auth();

        let treasury_info = TreasuryInfo {
            id: treasury_id.clone(),
            name,
            contract_address,
            creator,
            created_at: env.ledger().timestamp(),
            metadata,
        };

        // Store treasury info
        let key = (String::from_str(&env, "TREASURY"), treasury_id.clone());
        env.storage().persistent().set(&key, &treasury_info);

        // Add to treasury list
        let list_key = String::from_str(&env, "TREASURY_LIST");
        let mut list: Vec<String> = env.storage()
            .persistent()
            .get(&list_key)
            .unwrap_or(Vec::new(&env));
        
        list.push_back(treasury_id);
        env.storage().persistent().set(&list_key, &list);

        // Update stats
        Self::increment_treasury_count(env);
    }

    /// Get treasury information
    pub fn get_treasury(env: Env, treasury_id: String) -> Option<TreasuryInfo> {
        let key = (String::from_str(&env, "TREASURY"), treasury_id);
        env.storage().persistent().get(&key)
    }

    /// Get all registered treasuries
    pub fn get_all_treasuries(env: Env) -> Vec<String> {
        let list_key = String::from_str(&env, "TREASURY_LIST");
        env.storage()
            .persistent()
            .get(&list_key)
            .unwrap_or(Vec::new(&env))
    }

    /// Get treasuries by creator
    pub fn get_treasuries_by_creator(env: Env, creator: Address) -> Vec<String> {
        let all_treasuries = Self::get_all_treasuries(env.clone());
        let mut result = Vec::new(&env);

        for treasury_id in all_treasuries.iter() {
            if let Some(info) = Self::get_treasury(env.clone(), treasury_id.clone()) {
                if info.creator == creator {
                    result.push_back(treasury_id);
                }
            }
        }

        result
    }

    /// Update treasury metadata
    pub fn update_metadata(
        env: Env,
        treasury_id: String,
        updater: Address,
        new_metadata: String,
    ) {
        updater.require_auth();

        let key = (String::from_str(&env, "TREASURY"), treasury_id.clone());
        if let Some(mut info) = env.storage().persistent().get::<_, TreasuryInfo>(&key) {
            // Verify updater is the creator
            if info.creator != updater {
                panic!("Unauthorized");
            }

            info.metadata = new_metadata;
            env.storage().persistent().set(&key, &info);
        }
    }

    /// Get registry statistics
    pub fn get_stats(env: Env) -> RegistryStats {
        let stats_key = String::from_str(&env, "STATS");
        env.storage()
            .persistent()
            .get(&stats_key)
            .unwrap_or(RegistryStats {
                total_treasuries: 0,
                total_proposals: 0,
                active_treasuries: 0,
            })
    }

    /// Record proposal creation
    pub fn record_proposal(env: Env, treasury_id: String) {
        let key = (String::from_str(&env, "TREASURY"), treasury_id);
        if env.storage().persistent().has(&key) {
            Self::increment_proposal_count(env);
        }
    }

    // Internal helper functions

    fn increment_treasury_count(env: Env) {
        let stats_key = String::from_str(&env, "STATS");
        let mut stats = Self::get_stats(env.clone());
        stats.total_treasuries += 1;
        stats.active_treasuries += 1;
        env.storage().persistent().set(&stats_key, &stats);
    }

    fn increment_proposal_count(env: Env) {
        let stats_key = String::from_str(&env, "STATS");
        let mut stats = Self::get_stats(env.clone());
        stats.total_proposals += 1;
        env.storage().persistent().set(&stats_key, &stats);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_register_treasury() {
        let env = Env::default();
        env.mock_all_auths();

        let creator = Address::generate(&env);
        let contract_addr = Address::generate(&env);

        let client = RegistryContractClient::new(&env, &env.register_contract(None, RegistryContract));

        let treasury_id = String::from_str(&env, "treasury-1");
        let name = String::from_str(&env, "Test Treasury");
        let metadata = String::from_str(&env, "{}");

        client.register_treasury(&treasury_id, &name, &contract_addr, &creator, &metadata);

        let info = client.get_treasury(&treasury_id).unwrap();
        assert_eq!(info.name, name);
        assert_eq!(info.creator, creator);
    }

    #[test]
    fn test_get_all_treasuries() {
        let env = Env::default();
        env.mock_all_auths();

        let creator = Address::generate(&env);
        let contract_addr = Address::generate(&env);

        let client = RegistryContractClient::new(&env, &env.register_contract(None, RegistryContract));

        let treasury1 = String::from_str(&env, "treasury-1");
        let treasury2 = String::from_str(&env, "treasury-2");
        let name = String::from_str(&env, "Test");
        let metadata = String::from_str(&env, "{}");

        client.register_treasury(&treasury1, &name, &contract_addr, &creator, &metadata);
        client.register_treasury(&treasury2, &name, &contract_addr, &creator, &metadata);

        let all = client.get_all_treasuries();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_get_treasuries_by_creator() {
        let env = Env::default();
        env.mock_all_auths();

        let creator1 = Address::generate(&env);
        let creator2 = Address::generate(&env);
        let contract_addr = Address::generate(&env);

        let client = RegistryContractClient::new(&env, &env.register_contract(None, RegistryContract));

        let treasury1 = String::from_str(&env, "treasury-1");
        let treasury2 = String::from_str(&env, "treasury-2");
        let name = String::from_str(&env, "Test");
        let metadata = String::from_str(&env, "{}");

        client.register_treasury(&treasury1, &name, &contract_addr, &creator1, &metadata);
        client.register_treasury(&treasury2, &name, &contract_addr, &creator2, &metadata);

        let creator1_treasuries = client.get_treasuries_by_creator(&creator1);
        assert_eq!(creator1_treasuries.len(), 1);
        assert_eq!(creator1_treasuries.get(0).unwrap(), treasury1);
    }

    #[test]
    fn test_stats() {
        let env = Env::default();
        env.mock_all_auths();

        let creator = Address::generate(&env);
        let contract_addr = Address::generate(&env);

        let client = RegistryContractClient::new(&env, &env.register_contract(None, RegistryContract));

        let treasury_id = String::from_str(&env, "treasury-1");
        let name = String::from_str(&env, "Test");
        let metadata = String::from_str(&env, "{}");

        client.register_treasury(&treasury_id, &name, &contract_addr, &creator, &metadata);
        client.record_proposal(&treasury_id);

        let stats = client.get_stats();
        assert_eq!(stats.total_treasuries, 1);
        assert_eq!(stats.total_proposals, 1);
        assert_eq!(stats.active_treasuries, 1);
    }
}
