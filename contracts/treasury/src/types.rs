use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Treasury {
    pub id: String,
    pub name: String,
    pub creator: Address,
    pub signers: Vec<Address>,
    pub threshold: u32,
    pub created_at: u64,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Proposal {
    pub id: String,
    pub treasury_id: String,
    pub proposal_type: ProposalType,
    pub proposer: Address,
    pub status: ProposalStatus,
    pub approvals: Vec<Address>,
    pub rejections: Vec<Address>,
    pub created_at: u64,
    pub expires_at: u32,
    pub executed: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalType {
    Transfer {
        recipient: Address,
        asset: Address,
        amount: i128,
    },
    AddSigner {
        new_signer: Address,
    },
    RemoveSigner {
        signer: Address,
    },
    UpdateThreshold {
        new_threshold: u32,
    },
    Pause,
    Resume,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub address: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vote {
    pub proposal_id: String,
    pub voter: Address,
    pub approve: bool,
    pub timestamp: u64,
}
