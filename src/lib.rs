use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env,log, near_bindgen, AccountId, PromiseOrValue};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::json;
use near_sdk::json_types::{U128,U64};
use near_sdk::Promise;

near_sdk::setup_alloc!();

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
    pub loan_requested: U64,
    /// Current status of the loan
    pub status: LoanStatus,
    /// Submission time
    pub submission_time: U64,
    /// When somebody loaned.
    pub loan_time: Option<U64>,
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTLoans {
    pub last_loan_id: u64,
    pub loans: LookupMap<u64, Loan>,
}
impl Default for NFTLoans {
    fn default() -> Self {
        Self {
            last_loan_id: 0,
            loans: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl NFTLoans {
    // Receive an NFT with the method nft_transfer_call 
    // This method is called from the NFT contract
    // When transfered succesful it is saved as a new requesting for loaning
    pub fn nft_on_transfer(&mut self,sender_id: String,previous_owner_id: String,token_id: String,msg: String)->  bool{
        let signer_account_id = env::signer_account_id();
        env::log_str(
            &json!({
            "signer_id":signer_account_id.to_string(),
            "sender_id":sender_id,
            "previous_owner_id": previous_owner_id,
            "token_id": token_id,
            "msg":msg})
            .to_string(),
        );
        let id = self.last_loan_id;
        let signer_id = env::signer_account_id();
        let new_loan = Loan{
            nft_contract:signer_id,
            nft_id:token_id,
            nft_owner:signer_id ,
            description:Some("This a description".to_string()),
            loan_requested:U64(20),
            status: LoanStatus::Pending,
            submission_time: U64::from(env::block_timestamp()),
            loan_time:None,
        };
        self.loans.insert(&id, &new_loan);
        self.last_loan_id += 1;
        return false;
    }
    
    //View the loan_id of the last loan
    pub fn get_last_loan(&self)-> u64 {
        self.last_loan_id
    }

    pub fn get_nft_loan(&self, loan_id: u64) -> LoanOutput {
        let loans = self.loans.get(&loan_id).expect("ERR_NO_PROPOSAL");
        LoanOutput {
            id:loan_id,
            loan: loans.into(),
        }
    }


    /*
    //View wich NFT are available for loaning
    pub fn get_nfts_for_loan(&self, from_index: u64, limit: u64)-> Vec<LoanOutput> {
        let account_id = env::signer_account_id();
    }

    // Loan $NEAR Tokens to a loaning proposal
    #[payable]
    pub fn loan_for_nft(&mut self, loan_id: u64) -> Option<String> {
        //Review that NFT is still available for loaning
        assert_eq!(True,loan_status,"The NFT is not available for loaning");
        //Review that amount is the required
        assert_eq!(amount,amount_to_be_paid,"The amount payed is not equal as the requested");
        //Review that loaner is not the same as NFT owner
        assert_eq!(signer_account_id,nft_owner_id,"The owner cannot be the loaner");

        return self.records.get(&account_id);
    }

    //If time has passed and the NFT owner didn't pay
    //The loaner can claim the NFT and transfer to their wallet
    pub fn withdraw_nft_loaner(loan_id:u64) -> Option<String> {
        return self.records.get(&account_id);
    }

    //Canceled public offer for loaning
    pub fn withdraw_nft_owner(&self, loan_id: u64) -> Option<String> {
        return self.records.get(&account_id);
    }*/
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