use soroban_sdk::{Address, Env, String};
use crate::types::{Treasury, Proposal};
use crate::errors::TreasuryError;

const TREASURY_KEY: &str = "TREASURY";
const PROPOSAL_KEY: &str = "PROPOSAL";
const BALANCE_KEY: &str = "BALANCE";

// Treasury storage

pub fn write_treasury(env: &Env, treasury_id: &String, treasury: &Treasury) {
    let key = (TREASURY_KEY, treasury_id.clone());
    env.storage().persistent().set(&key, treasury);
}

pub fn read_treasury(env: &Env, treasury_id: &String) -> Result<Treasury, TreasuryError> {
    let key = (TREASURY_KEY, treasury_id.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(TreasuryError::TreasuryNotFound)
}

pub fn has_treasury(env: &Env, treasury_id: &String) -> bool {
    let key = (TREASURY_KEY, treasury_id.clone());
    env.storage().persistent().has(&key)
}

// Proposal storage

pub fn write_proposal(env: &Env, proposal_id: &String, proposal: &Proposal) {
    let key = (PROPOSAL_KEY, proposal_id.clone());
    env.storage().persistent().set(&key, proposal);
}

pub fn read_proposal(env: &Env, proposal_id: &String) -> Result<Proposal, TreasuryError> {
    let key = (PROPOSAL_KEY, proposal_id.clone());
    env.storage()
        .persistent()
        .get(&key)
        .ok_or(TreasuryError::ProposalNotFound)
}

pub fn has_proposal(env: &Env, proposal_id: &String) -> bool {
    let key = (PROPOSAL_KEY, proposal_id.clone());
    env.storage().persistent().has(&key)
}

// Balance storage

pub fn get_balance(env: &Env, treasury_id: &String, asset: &Address) -> i128 {
    let key = (BALANCE_KEY, treasury_id.clone(), asset.clone());
    env.storage().persistent().get(&key).unwrap_or(0)
}

pub fn add_balance(env: &Env, treasury_id: &String, asset: &Address, amount: i128) {
    let current = get_balance(env, treasury_id, asset);
    let new_balance = current + amount;
    let key = (BALANCE_KEY, treasury_id.clone(), asset.clone());
    env.storage().persistent().set(&key, &new_balance);
}

pub fn sub_balance(env: &Env, treasury_id: &String, asset: &Address, amount: i128) {
    let current = get_balance(env, treasury_id, asset);
    let new_balance = current - amount;
    let key = (BALANCE_KEY, treasury_id.clone(), asset.clone());
    env.storage().persistent().set(&key, &new_balance);
}
