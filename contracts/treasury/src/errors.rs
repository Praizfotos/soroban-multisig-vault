use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum TreasuryError {
    // Treasury errors
    TreasuryNotFound = 1,
    TreasuryExists = 2,
    TreasuryPaused = 3,
    TreasuryNotPaused = 4,
    
    // Authorization errors
    Unauthorized = 10,
    
    // Signer errors
    InvalidSigners = 20,
    SignerExists = 21,
    SignerNotFound = 22,
    
    // Threshold errors
    InvalidThreshold = 30,
    
    // Proposal errors
    ProposalNotFound = 40,
    ProposalExists = 41,
    ProposalExpired = 42,
    ProposalExecuted = 43,
    ProposalNotApproved = 44,
    InvalidProposalStatus = 45,
    InvalidExpiration = 46,
    
    // Voting errors
    AlreadyVoted = 50,
    
    // Balance errors
    InsufficientBalance = 60,
    
    // General errors
    InvalidInput = 100,
}
