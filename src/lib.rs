use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue,PanicOnDefault};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{json,from_str};
use near_sdk::Promise;
use uint::construct_uint;

use std::cmp::min;

near_sdk::setup_alloc!();

pub type EpochHeight = u64;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}
//aqui van los nombres de los metodos que mandaremos llamar
#[ext_contract(ext_contract_nft)]
trait NonFungibleToken {

    // change methods
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        msg: String,
    );

}
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
    // Transaction interest estimated for the NFT payment
    // It is based as 10000=100%
    pub contract_interest: u64,
    pub loans: LookupMap<u64, Loan>,
    /// Total token amount deposited.
    pub total_amount: Balance,
    /// Duration of payment period for loans
    pub payment_period: u64,
    /// Fee payed to Nativo Loans
    pub contract_fee:u64, //200=2%
}

#[near_bindgen]
impl NFTLoans {
    //Initialize the contract
    #[init]
    pub fn new(
        owner_account_id: AccountId,
        treasury_account_id: AccountId,
        contract_interest: u64, //800=8%
        contract_fee: u64, //200=2%
        
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        let result= Self{
            owner_account_id,
            treasury_account_id,
            last_loan_id: 0,
            contract_interest,
            loans: LookupMap::new(b"r".to_vec()),
            total_amount: 0,
            payment_period:1_000_000_000 * 60 * 60 * 24 * 7,
            contract_fee, //200=2%
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
            None
        };*/
        //assert!(msg.is_empty() || msg=="" ,"ERR_INVALID_MESSAGE");
        let id = self.last_loan_id;
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();

        //calculate amount to be payed 
        let amount_to_loaner:u128 = u128::from(msg_json.loan_amount_requested)+(u128::from(msg_json.loan_amount_requested)*u128::from(self.contract_interest)/10000);
        env::log_str(&amount_to_loaner.to_string());

        let new_loan = Loan{
            nft_contract:contract_id,
            nft_id:token_id,
            nft_owner:signer_id ,
            description:msg_json.description,
            loan_requested:msg_json.loan_amount_requested,
            loan_payback:amount_to_loaner,
            status: LoanStatus::Pending,
            submission_time: env::block_timestamp(),
            loan_time:None,
            loan_deadline:None,
            loaner_id:None,
        };
        self.loans.insert(&id, &new_loan);
        self.last_loan_id += 1;
        /*env::log_str(
            &json!(new_loan)
            .to_string(),
        );*/
        
        //If for some reason the contract failed it need to returns the NFT to the original owner (true)
        return PromiseOrValue::Value(false);
    }

    // Loan $NEAR Tokens to a loaning proposal
    #[payable]
    pub fn loan_for_nft(&mut self, loan_id: u64) -> Option<Loan> {
        let mut loan:Loan = self.loans.get(&loan_id).expect("the token doesn't have an active loan"); ///use a expect and explain that the loan wasnt found        
        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();

        //Review that NFT is still available for loaning
        assert_eq!(LoanStatus::Pending,loan.status,"The NFT is not available for loaning");
        //Review that amount is the required
        assert_eq!(attached_deposit,loan.loan_requested,"The amount payed is not equal as the requested");
        //Review that loaner is not the same as NFT owner
        assert_ne!(signer_id,loan.nft_owner,"The owner cannot be the loaner");

        loan.status=LoanStatus::Loaned;
        loan.loaner_id = Some(signer_id);
        loan.loan_time = Some(env::block_timestamp());
        //loan.loan_deadline = Some(env::block_timestamp()+60);
        loan.loan_deadline = Some(env::block_timestamp()+self.payment_period);

        let nft_owner = loan.nft_owner.clone();

        //here is pending of remove the 2% and be transfered to treasury

        //Here is removed % fee from amount transfered to owner
        let amount_to_owner:u128 = u128::from(loan.loan_requested)-(u128::from(loan.loan_requested)*u128::from(self.contract_fee)/10000);

        // Here is calculated the amount that will be sended to the treasury 
        let amount_to_treasury:u128 = u128::from(loan.loan_requested)*u128::from(self.contract_fee)/10000;

        //Transfers are done
        Promise::new(nft_owner).transfer(amount_to_owner); //before the owner recived the amount for treasury
        Promise::new(self.treasury_account_id.clone()).transfer(amount_to_treasury);//before the treasury recived the amount for owner

        self.loans.insert(&loan_id, &loan);
        return Some(loan);
    }

    #[payable]
    pub fn pay_loan(&mut self, loan_id: u64) -> Option<Loan> {
        let mut loan:Loan = self.loans.get(&loan_id).unwrap();
        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();
        let time_stamp=env::block_timestamp();


        //Review that NFT is still available for loaning
        assert_eq!(LoanStatus::Loaned,loan.status,"The NFT is not loaned");
        //Review that amount is the required
        //Here is pending of calculate the % of interest
        assert_eq!(attached_deposit,loan.loan_payback,"The amount payed is not equal as the requested");
        //Review that loaner is not the same as NFT owner
        assert_eq!(signer_id,loan.nft_owner,"The payer should be the owner");
        //Review that loaner is not the same as NFT owner
        env::log_str(&time_stamp.to_string());
        env::log_str(&loan.loan_deadline.unwrap().to_string());
        assert_eq!(time_stamp<=loan.loan_deadline.unwrap(),true,"The payment loan time has expired");

        //Here is pending of calculate the % of interest 
        Promise::new(loan.loaner_id.clone().unwrap()).transfer(u128::from(attached_deposit));
        // Inside a contract function on ContractA, a cross contract call is started
        // From ContractA to ContractB
        ext_contract_nft::nft_transfer(
        signer_id,
        loan.nft_id.clone().to_string(),
        "Withdraw of NFT from Nativo Loans".to_string(),
        loan.nft_contract.clone(), // contract account id
        1, // yocto NEAR to attach
        Gas::from(5_000_000_000_000) // gas to attach
        );

        loan.status=LoanStatus::Payed;
        self.loans.insert(&loan_id, &loan);

        return Some(loan);
    }

    //Canceled public offer for loaning
    #[payable]
    pub fn withdraw_nft_owner(&mut self, loan_id: u64){
        let mut loan:Loan = self.loans.get(&loan_id).unwrap();
        let signer_id =env::signer_account_id();
        let deposit = env::attached_deposit();

        //assert!(env::block_timestamp()<=loan.loan_time.unwrap()+self.payment_period&&loan.status==LoanStatus::Loaned,"The NFT is still pending of get loan payed");

        assert!(loan.status!=LoanStatus::Canceled,"The loan is canceled.");

        //Review that claimer is the same as NFT owner
        //assert_ne!(signer_id,loan.nft_owner,"You are not the owner of this NFT");

        if signer_id != loan.nft_owner.clone(){
            env::panic_str("You are not the owner of this NFT");
        }

        loan.status=LoanStatus::Canceled;
        self.loans.insert(&loan_id, &loan);
        // env::log_str(
        //     &json!(&loan)
        //     .to_string(),
        // );

        // Inside a contract function on ContractA, a cross contract call is started
        // From ContractA to ContractB
        ext_contract_nft::nft_transfer(
        signer_id,
        loan.nft_id.to_string(),
        "Withdraw of NFT from Nativo Loans".to_string(),
        loan.nft_contract, // contract account id
        deposit, // yocto NEAR to attach
        Gas::from(5_000_000_000_000) // gas to attach
        );
        /*
        // When the cross contract call from A to B finishes the my_callback method is triggered.
        // Since my_callback is a callback, it will have access to the returned data from B
        .then(ext_self::my_callback(
        &env::current_account_id(), // this contract’s account id
        0, // yocto NEAR to attach to the callback
        5_000_000_000_000 // gas to attach to the callback
        ))*/
    }

    //View the loan_id of the last loan
    pub fn get_contract_interest(&self)-> u64 {
        self.contract_interest
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
    #[payable]
    pub fn withdraw_nft_loaner(&mut self,loan_id:u64){
        let mut loan:Loan = self.loans.get(&loan_id).unwrap();
        let signer_id=env::signer_account_id();
        let time_stamp=env::block_timestamp();
        let deposit = env::attached_deposit();

        assert_eq!(time_stamp>=loan.loan_deadline.unwrap(),true,"The payment loan time has not expired");
        

        //assert!(loan.status!=LoanStatus::Loaned,"The NFT is under a loaning process.");

        //Review that claimer is the same as NFT loaner
        if signer_id != loan.loaner_id.clone().unwrap(){
            env::panic_str("You are not the loaner of this NFT");
        }

        loan.status=LoanStatus::Expired;
        self.loans.insert(&loan_id, &loan);
        // env::log_str(
        //     &json!(&loan)
        //     .to_string(),
        // );

        // Inside a contract function on ContractA, a cross contract call is started
        // From ContractA to ContractB
        ext_contract_nft::nft_transfer(
        signer_id,
        loan.nft_id.to_string(),
        "Withdraw of NFT from Nativo Loans".to_string(),
        loan.nft_contract, // contract account id
        deposit, // yocto NEAR to attach
        Gas::from(5_000_000_000_000) // gas to attach
        );
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