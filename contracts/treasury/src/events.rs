use soroban_sdk::{symbol_short, Address, Env, String};

// Treasury events

pub fn treasury_created(
    env: &Env,
    treasury_id: &String,
    name: &String,
    creator: &Address,
    signer_count: u32,
    threshold: u32,
) {
    env.events().publish(
        (symbol_short!("treasury"), symbol_short!("created")),
        (treasury_id, name, creator, signer_count, threshold),
    );
}

pub fn treasury_paused(env: &Env, treasury_id: &String) {
    env.events().publish(
        (symbol_short!("treasury"), symbol_short!("paused")),
        treasury_id,
    );
}

pub fn treasury_resumed(env: &Env, treasury_id: &String) {
    env.events().publish(
        (symbol_short!("treasury"), symbol_short!("resumed")),
        treasury_id,
    );
}

// Deposit events

pub fn deposit_received(
    env: &Env,
    treasury_id: &String,
    asset: &Address,
    amount: i128,
    depositor: &Address,
) {
    env.events().publish(
        (symbol_short!("deposit"), symbol_short!("received")),
        (treasury_id, asset, amount, depositor),
    );
}

// Proposal events

pub fn proposal_created(
    env: &Env,
    proposal_id: &String,
    treasury_id: &String,
    proposer: &Address,
) {
    env.events().publish(
        (symbol_short!("proposal"), symbol_short!("created")),
        (proposal_id, treasury_id, proposer),
    );
}

pub fn proposal_executed(env: &Env, proposal_id: &String) {
    env.events().publish(
        (symbol_short!("proposal"), symbol_short!("executed")),
        proposal_id,
    );
}

// Vote events

pub fn vote_cast(env: &Env, proposal_id: &String, voter: &Address, approve: bool) {
    env.events().publish(
        (symbol_short!("vote"), symbol_short!("cast")),
        (proposal_id, voter, approve),
    );
}

// Signer events

pub fn signer_added(env: &Env, treasury_id: &String, signer: &Address) {
    env.events().publish(
        (symbol_short!("signer"), symbol_short!("added")),
        (treasury_id, signer),
    );
}

pub fn signer_removed(env: &Env, treasury_id: &String, signer: &Address) {
    env.events().publish(
        (symbol_short!("signer"), symbol_short!("removed")),
        (treasury_id, signer),
    );
}

// Threshold events

pub fn threshold_updated(env: &Env, treasury_id: &String, new_threshold: u32) {
    env.events().publish(
        (symbol_short!("thresh"), symbol_short!("updated")),
        (treasury_id, new_threshold),
    );
}
