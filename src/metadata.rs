use crate::*;

use std::mem::size_of;

pub type LoanId = u64;


/// Status of a loan.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum LoanStatus {
    //First status when no body had loaned for this NFT
    Pending,
    /// If somebody loaned for this NFT
    Loaned,
    /// Expired after period of time. Loaner claimed the NFT.
    Expired,
    /// If NFT owner payed back for the loan
    Payed,
    // If no body loaned for this NFT. This status gets after owners claim back its NFT.
    Canceled,
}

/// Proposal for loaning that are sent to this DAO.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct Loan {
    /// Original nft owner.
    pub nft_owner: AccountId,
    /// Original nft contract.
    pub nft_contract: AccountId,
    /// NFT id in origin contract.
    pub nft_id: String,
    /// Description of this loan.
    pub description: Option<String>,
    /// loan amount requested
    pub loan_requested: u128,
    /// loan amount that have to be payback
    pub loan_payback: u128,
    /// Current status of the loan
    pub status: LoanStatus,
    /// Submission time
    pub submission_time: EpochHeight,
    /// When somebody loaned.
    pub loan_time: Option<EpochHeight>,
    /// When will the loaning end and the loaner can withdraw the NFT
    /// Also is the deadline when NFT owner can payback
    pub loan_deadline: Option<EpochHeight>,
    /// When somebody loaned.
    pub loaner_id: Option<AccountId>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize,Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Metrics {
    //Index for loans
    pub total_loans: u64,
    //how much loans are running
    pub total_loans_active: u128,
    /// Total token amount deposited.
    pub total_amount_payed: u128,
    // a flag to start/stop the ntv minting
    pub ntv_status:bool,
    //multuplier for ntv tokens.
    pub ntv_multiply:u128,
    //how much money has made by auctions
    pub total_amount_lent: u128,
    //how much ATH has made by auctions
    pub loan_current_ath: u128,
}

/// This is format of output via JSON for the Loan.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LoanOutput {
    /// Id of the Loan.
    pub id: LoanId,
    #[serde(flatten)]
    pub loan: Loan,
}
/// This is format of output via JSON for the loan message.
#[derive( Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
    pub description: Option<String>,
    pub loan_amount_requested: u128,
}


