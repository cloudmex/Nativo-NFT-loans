use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap,UnorderedSet};
use near_sdk::{env,log, Duration, Balance, near_bindgen, AccountId, PromiseOrValue,PanicOnDefault};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json,from_str};
use near_sdk::json_types::{U64};
use near_sdk::Promise;
use uint::construct_uint;

use std::cmp::min;

near_sdk::setup_alloc!();

pub type EpochHeight = u64;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

/// Status of a loan.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum LoanStatus {
    //First status when no body had loaned for this NFT
    Pending,
    /// If somebody loaned for this NFT
    Loaned,
    /// Expired after period of time. Loaner can claim the NFT.
    Expired,
    /// If NFT owner payed back for the loan
    Payed,
    // If no body loaned for this NFT. This status gets after owners can claim back their NFT.
    Failed,
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
    /// Current status of the loan
    pub loan_requested: u128,
    /// Current status of the loan
    pub status: LoanStatus,
    /// Submission time
    pub submission_time: EpochHeight,
    /// When somebody loaned.
    pub loan_time: Option<EpochHeight>,
    /// When somebody loaned.
    pub loaner_id: Option<AccountId>,
}
/// This is format of output via JSON for the Loan.
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct LoanOutput {
    /// Id of the Loan.
    pub id: u64,
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct NFTLoans {
    /// Owner's account ID (it will be a DAO on phase II)
    pub owner_account_id: AccountId,
    /// Owner's account ID (it will be a DAO on phase II)
    pub treasury_account_id: AccountId,
    //Index for loans
    pub last_loan_id: u64,
    // APY estimated for the NFT payment
    pub contract_apy: u64,
    pub loans: LookupMap<u64, Loan>,
    /// Total token amount deposited.
    pub total_amount: Balance,
    /// Duration of payment period for loans
    pub payment_period: u64,
}

#[near_bindgen]
impl NFTLoans {
    //Initialize the contract
    #[init]
    pub fn new(
        owner_account_id: AccountId,
        treasury_account_id: AccountId,
        contract_apy: u64,
        
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result= Self{
            owner_account_id,
            treasury_account_id,
            last_loan_id: 0,
            contract_apy,
            loans: LookupMap::new(b"r".to_vec()),
            total_amount: 0,
            payment_period:1_000_000_000 * 60 * 60 * 24 * 7,
        };
        return result;
    }
    // Receive an NFT with the method nft_transfer_call 
    // This method is called from the NFT contract
    // When transfered succesful it is saved as a new requesting for loaning
    pub fn nft_on_transfer(&mut self,sender_id: AccountId,previous_owner_id: AccountId,token_id: String,msg: String)  -> PromiseOrValue<bool>{
        env::log_str(&msg.to_string());
        /*if msg.is_empty() || msg=="" {
            env::log_str("ERR_INVALID_MESSAGE");
            PromiseOrValue::Value(true);
        };*/
        //assert!(msg.is_empty() || msg=="" ,"ERR_INVALID_MESSAGE");
        let id = self.last_loan_id;
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();

        let new_loan = Loan{
            nft_contract:contract_id,
            nft_id:token_id,
            nft_owner:signer_id ,
            description:msg_json.description,
            loan_requested:msg_json.loan_amount_requested,
            status: LoanStatus::Pending,
            submission_time: env::block_timestamp(),
            loan_time:None,
            loaner_id:None,
        };
        self.loans.insert(&id, &new_loan);
        self.last_loan_id += 1;
        /*env::log_str(
            &json!(new_loan)
            .to_string(),
        );
        */
        //If for some reason the contract failed it need to returns the NFT to the original owner (true)
        return PromiseOrValue::Value(true);
    }
    // Loan $NEAR Tokens to a loaning proposal
    #[payable]
    pub fn loan_for_nft(&mut self, loan_id: u64) -> Option<Loan> {
        let mut loan:Loan = self.loans.get(&loan_id).unwrap();
        let signer_id =env::signer_account_id();

        //Review that NFT is still available for loaning
        assert_eq!(LoanStatus::Pending,loan.status,"The NFT is not available for loaning");
        //Review that amount is the required
        assert_eq!(env::attached_deposit(),loan.loan_requested,"The amount payed is not equal as the requested");
        //Review that loaner is not the same as NFT owner
        assert_ne!(signer_id,loan.nft_owner,"The owner cannot be the loaner");

        loan.status=LoanStatus::Loaned;
        loan.loaner_id=Some(signer_id);
        loan.loan_time=Some(env::block_timestamp());
        self.loans.insert(&loan_id, &loan);

        return Some(loan);
    }

    #[payable]
    pub fn pay_loan(&mut self, loan_id: u64) -> Option<Loan> {
        let mut loan:Loan = self.loans.get(&loan_id).unwrap();
        let signer_id =env::signer_account_id();

        //Review that NFT is still available for loaning
        assert_eq!(LoanStatus::Loaned,loan.status,"The NFT is not loaned");
        //Review that amount is the required
        assert_eq!(env::attached_deposit(),loan.loan_requested,"The amount payed is not equal as the requested");
        //Review that loaner is not the same as NFT owner
        assert_eq!(signer_id,loan.nft_owner,"The owner cannot be the loaner");
        //Review that loaner is not the same as NFT owner
        assert!(env::block_timestamp()>=loan.loan_time.unwrap()+self.payment_period,"The payment loan time has expired");

        loan.status=LoanStatus::Payed;

        return Some(loan);
    }
    
    pub fn apply_pct(basis_points: u16, amount: u128) -> u128 {
        return ((U256::from(basis_points) * U256::from(amount) / U256::from(10_000))+U256::from(amount)).as_u128();
    }
    

    //View the loan_id of the last loan
    pub fn get_contract_apy(&self)-> u64 {
        self.contract_apy
    }
    
    //View the loan_id of the last loan
    pub fn get_last_loan(&self)-> u64 {
        self.last_loan_id
    }

    pub fn get_nft_loan(&self, loan_id: u64) -> LoanOutput {
        let loans = self.loans.get(&loan_id).expect("ERR_NO_LOAN");
        LoanOutput {
            id:loan_id,
            loan: loans.into(),
        }
    }
    //View wich NFT are available for loaning
    pub fn get_nfts_for_loan(&self, from_index: u64, limit: u64)-> Vec<LoanOutput> {
        (from_index..min(self.last_loan_id, from_index + limit))
            .filter_map(|id| {
                self.loans.get(&id).map(|loan| LoanOutput {
                    id,
                    loan: loan.into(),
                })
            })
            .collect()
    }
   
    //If time has passed and the NFT owner didn't pay
    //The loaner can claim the NFT and transfer to their wallet
    pub fn withdraw_nft_loaner(&self,loan_id:u64){
        //return self.loans.get(&loan_id);
    }

    //Canceled public offer for loaning
    pub fn withdraw_nft_owner(&self, loan_id: u64){
        //return self.loans.get(&loan_id);
    }


    /**/
}


// This are the tests
// PENDING
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn set_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = StatusMessage::default();
        contract.set_status("hello".to_string());
        assert_eq!(
            "hello".to_string(),
            contract.get_status("bob_near".to_string()).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = StatusMessage::default();
        assert_eq!(None, contract.get_status("francis.near".to_string()));
    }
}