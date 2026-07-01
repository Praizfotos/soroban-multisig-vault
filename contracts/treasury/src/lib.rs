#![no_std]

mod storage;
mod types;
mod events;
mod errors;

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env, String, Vec};
use types::{Treasury, Proposal, ProposalType, ProposalStatus, Asset};
use errors::TreasuryError;

#[contract]
pub struct TreasuryContract;

#[contractimpl]
impl TreasuryContract {
    /// Initialize a new treasury
    pub fn initialize(
        env: Env,
        treasury_id: String,
        name: String,
        creator: Address,
        signers: Vec<Address>,
        threshold: u32,
    ) -> Result<(), TreasuryError> {
        // Verify creator authorization
        creator.require_auth();

        // Validate inputs
        if signers.len() == 0 {
            return Err(TreasuryError::InvalidSigners);
        }

        if threshold == 0 || threshold > signers.len() {
            return Err(TreasuryError::InvalidThreshold);
        }

        // Check if treasury already exists
        if storage::has_treasury(&env, &treasury_id) {
            return Err(TreasuryError::TreasuryExists);
        }

        // Create treasury
        let treasury = Treasury {
            id: treasury_id.clone(),
            name: name.clone(),
            creator: creator.clone(),
            signers: signers.clone(),
            threshold,
            created_at: env.ledger().timestamp(),
            paused: false,
        };

        // Store treasury
        storage::write_treasury(&env, &treasury_id, &treasury);

        // Emit event
        events::treasury_created(&env, &treasury_id, &name, &creator, signers.len(), threshold);

        Ok(())
    }

    /// Deposit assets into treasury
    pub fn deposit(
        env: Env,
        treasury_id: String,
        asset: Address,
        amount: i128,
        depositor: Address,
    ) -> Result<(), TreasuryError> {
        depositor.require_auth();

        // Verify treasury exists and is not paused
        let treasury = storage::read_treasury(&env, &treasury_id)?;
        if treasury.paused {
            return Err(TreasuryError::TreasuryPaused);
        }

        // Transfer tokens to contract
        let token_client = token::Client::new(&env, &asset);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        // Update balance
        storage::add_balance(&env, &treasury_id, &asset, amount);

        // Emit event
        events::deposit_received(&env, &treasury_id, &asset, amount, &depositor);

        Ok(())
    }

    /// Create a new proposal
    pub fn create_proposal(
        env: Env,
        treasury_id: String,
        proposal_id: String,
        proposer: Address,
        proposal_type: ProposalType,
        expiration_ledger: u32,
    ) -> Result<(), TreasuryError> {
        proposer.require_auth();

        // Verify treasury exists and proposer is a signer
        let treasury = storage::read_treasury(&env, &treasury_id)?;
        
        if !treasury.signers.contains(&proposer) {
            return Err(TreasuryError::Unauthorized);
        }

        if treasury.paused && !matches!(proposal_type, ProposalType::Resume) {
            return Err(TreasuryError::TreasuryPaused);
        }

        // Check if proposal already exists
        if storage::has_proposal(&env, &proposal_id) {
            return Err(TreasuryError::ProposalExists);
        }

        // Validate expiration
        if expiration_ledger <= env.ledger().sequence() {
            return Err(TreasuryError::InvalidExpiration);
        }

        // Create proposal
        let proposal = Proposal {
            id: proposal_id.clone(),
            treasury_id: treasury_id.clone(),
            proposal_type,
            proposer: proposer.clone(),
            status: ProposalStatus::Pending,
            approvals: Vec::new(&env),
            rejections: Vec::new(&env),
            created_at: env.ledger().timestamp(),
            expires_at: expiration_ledger,
            executed: false,
        };

        // Store proposal
        storage::write_proposal(&env, &proposal_id, &proposal);

        // Emit event
        events::proposal_created(&env, &proposal_id, &treasury_id, &proposer);

        Ok(())
    }

    /// Vote on a proposal
    pub fn vote(
        env: Env,
        proposal_id: String,
        voter: Address,
        approve: bool,
    ) -> Result<(), TreasuryError> {
        voter.require_auth();

        // Load proposal
        let mut proposal = storage::read_proposal(&env, &proposal_id)?;

        // Verify proposal is still valid
        if proposal.executed {
            return Err(TreasuryError::ProposalExecuted);
        }

        if env.ledger().sequence() > proposal.expires_at {
            return Err(TreasuryError::ProposalExpired);
        }

        if proposal.status != ProposalStatus::Pending {
            return Err(TreasuryError::InvalidProposalStatus);
        }

        // Verify voter is a signer
        let treasury = storage::read_treasury(&env, &proposal.treasury_id)?;
        if !treasury.signers.contains(&voter) {
            return Err(TreasuryError::Unauthorized);
        }

        // Prevent double voting
        if proposal.approvals.contains(&voter) || proposal.rejections.contains(&voter) {
            return Err(TreasuryError::AlreadyVoted);
        }

        // Record vote
        if approve {
            proposal.approvals.push_back(voter.clone());
        } else {
            proposal.rejections.push_back(voter.clone());
        }

        // Check if threshold reached
        if proposal.approvals.len() >= treasury.threshold {
            proposal.status = ProposalStatus::Approved;
        } else if proposal.rejections.len() > (treasury.signers.len() - treasury.threshold) {
            proposal.status = ProposalStatus::Rejected;
        }

        // Update proposal
        storage::write_proposal(&env, &proposal_id, &proposal);

        // Emit event
        events::vote_cast(&env, &proposal_id, &voter, approve);

        // Auto-execute if approved
        if proposal.status == ProposalStatus::Approved {
            Self::execute_proposal(env.clone(), proposal_id)?;
        }

        Ok(())
    }

    /// Execute an approved proposal
    pub fn execute_proposal(
        env: Env,
        proposal_id: String,
    ) -> Result<(), TreasuryError> {
        // Load proposal
        let mut proposal = storage::read_proposal(&env, &proposal_id)?;

        // Verify proposal can be executed
        if proposal.executed {
            return Err(TreasuryError::ProposalExecuted);
        }

        if proposal.status != ProposalStatus::Approved {
            return Err(TreasuryError::ProposalNotApproved);
        }

        if env.ledger().sequence() > proposal.expires_at {
            return Err(TreasuryError::ProposalExpired);
        }

        // Execute based on proposal type
        match &proposal.proposal_type {
            ProposalType::Transfer { recipient, asset, amount } => {
                Self::execute_transfer(&env, &proposal.treasury_id, recipient, asset, *amount)?;
            }
            ProposalType::AddSigner { new_signer } => {
                Self::execute_add_signer(&env, &proposal.treasury_id, new_signer)?;
            }
            ProposalType::RemoveSigner { signer } => {
                Self::execute_remove_signer(&env, &proposal.treasury_id, signer)?;
            }
            ProposalType::UpdateThreshold { new_threshold } => {
                Self::execute_update_threshold(&env, &proposal.treasury_id, *new_threshold)?;
            }
            ProposalType::Pause => {
                Self::execute_pause(&env, &proposal.treasury_id)?;
            }
            ProposalType::Resume => {
                Self::execute_resume(&env, &proposal.treasury_id)?;
            }
        }

        // Mark as executed
        proposal.executed = true;
        storage::write_proposal(&env, &proposal_id, &proposal);

        // Emit event
        events::proposal_executed(&env, &proposal_id);

        Ok(())
    }

    /// Get treasury details
    pub fn get_treasury(env: Env, treasury_id: String) -> Result<Treasury, TreasuryError> {
        storage::read_treasury(&env, &treasury_id)
    }

    /// Get proposal details
    pub fn get_proposal(env: Env, proposal_id: String) -> Result<Proposal, TreasuryError> {
        storage::read_proposal(&env, &proposal_id)
    }

    /// Get treasury balance for an asset
    pub fn get_balance(env: Env, treasury_id: String, asset: Address) -> Result<i128, TreasuryError> {
        let treasury = storage::read_treasury(&env, &treasury_id)?;
        Ok(storage::get_balance(&env, &treasury_id, &asset))
    }

    // Internal execution functions

    fn execute_transfer(
        env: &Env,
        treasury_id: &String,
        recipient: &Address,
        asset: &Address,
        amount: i128,
    ) -> Result<(), TreasuryError> {
        let balance = storage::get_balance(env, treasury_id, asset);
        if balance < amount {
            return Err(TreasuryError::InsufficientBalance);
        }

        let token_client = token::Client::new(env, asset);
        token_client.transfer(&env.current_contract_address(), recipient, &amount);

        storage::sub_balance(env, treasury_id, asset, amount);

        Ok(())
    }

    fn execute_add_signer(
        env: &Env,
        treasury_id: &String,
        new_signer: &Address,
    ) -> Result<(), TreasuryError> {
        let mut treasury = storage::read_treasury(env, treasury_id)?;

        if treasury.signers.contains(new_signer) {
            return Err(TreasuryError::SignerExists);
        }

        treasury.signers.push_back(new_signer.clone());
        storage::write_treasury(env, treasury_id, &treasury);

        events::signer_added(env, treasury_id, new_signer);

        Ok(())
    }

    fn execute_remove_signer(
        env: &Env,
        treasury_id: &String,
        signer: &Address,
    ) -> Result<(), TreasuryError> {
        let mut treasury = storage::read_treasury(env, treasury_id)?;

        if !treasury.signers.contains(signer) {
            return Err(TreasuryError::SignerNotFound);
        }

        // Ensure threshold is still valid after removal
        if treasury.signers.len() - 1 < treasury.threshold {
            return Err(TreasuryError::InvalidThreshold);
        }

        treasury.signers.retain(|s| s != signer);
        storage::write_treasury(env, treasury_id, &treasury);

        events::signer_removed(env, treasury_id, signer);

        Ok(())
    }

    fn execute_update_threshold(
        env: &Env,
        treasury_id: &String,
        new_threshold: u32,
    ) -> Result<(), TreasuryError> {
        let mut treasury = storage::read_treasury(env, treasury_id)?;

        if new_threshold == 0 || new_threshold > treasury.signers.len() {
            return Err(TreasuryError::InvalidThreshold);
        }

        treasury.threshold = new_threshold;
        storage::write_treasury(env, treasury_id, &treasury);

        events::threshold_updated(env, treasury_id, new_threshold);

        Ok(())
    }

    fn execute_pause(
        env: &Env,
        treasury_id: &String,
    ) -> Result<(), TreasuryError> {
        let mut treasury = storage::read_treasury(env, treasury_id)?;

        if treasury.paused {
            return Err(TreasuryError::TreasuryPaused);
        }

        treasury.paused = true;
        storage::write_treasury(env, treasury_id, &treasury);

        events::treasury_paused(env, treasury_id);

        Ok(())
    }

    fn execute_resume(
        env: &Env,
        treasury_id: &String,
    ) -> Result<(), TreasuryError> {
        let mut treasury = storage::read_treasury(env, treasury_id)?;

        if !treasury.paused {
            return Err(TreasuryError::TreasuryNotPaused);
        }

        treasury.paused = false;
        storage::write_treasury(env, treasury_id, &treasury);

        events::treasury_resumed(env, treasury_id);

        Ok(())
    }
}

#[cfg(test)]
mod test;
