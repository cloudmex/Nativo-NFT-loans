use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::{env,ext_contract, Balance,Gas, near_bindgen, AccountId, PromiseOrValue,PanicOnDefault,CryptoHash};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{U128};
use near_sdk::serde_json::{json,from_str};
use near_sdk::Promise;
use uint::construct_uint;

use std::cmp::min;

use crate::internal::*;
pub use crate::metadata::*;
pub use crate::migrate::*;

mod enumeration;
mod metadata;
mod internal;
mod migrate;

near_sdk::setup_alloc!();

#[ext_contract(ext_nft)]
pub trait ExternsContract {
    fn mint(&self, account_id:AccountId,amount: String) -> String;
    // fn reward_player(&self,player_owner_id: String,tokens_mint: String) -> String;
}

pub type EpochHeight = u64;
const NTVTOKEN_CONTRACT:  &str = "nativo_token.testnet";

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}
/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    LoansPerOwner,
    LoanPerOwnerInner { account_id_hash: CryptoHash },
    LoansPerLender,
    LoanPerLenderInner { account_id_hash: CryptoHash },
    LoansById,
    LoansMetadataById,
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
    //keeps track of the loan struct for a given loan ID
    pub loans_by_id: UnorderedMap<LoanId, Loan>,
    //keeps track of all the loan IDs for a given account
    pub loans_per_owner: LookupMap<AccountId, UnorderedSet<LoanId>>,
    //keeps track of all the loan IDs for a given account
    pub loans_per_lender: LookupMap<AccountId, UnorderedSet<LoanId>>,
    /// Total token amount lent.
    pub total_amount_lent: u128,
    /// Total token amount payed.
    pub total_amount_payed: u128,
    ///multuplier for ntv tokens.
    pub ntv_multiply:u128,
    /// Duration of payment period for loans
    pub payment_period: u64,
    /// Fee payed to Nativo Loans
    pub contract_fee:u64, //200=2%
    /// If minting ntv is enabled
    pub is_minting_ntv: bool,
    // loan current ath
    pub loan_current_ath: u128,
    /// loans active
    pub loans_active: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldNFTLoans {
    /// Owner's account ID (it will be a DAO on phase II)
    pub owner_account_id: AccountId,
    /// Owner's account ID (it will be a DAO on phase II)
    pub treasury_account_id: AccountId,
    //Index for loans
    pub last_loan_id: u64,
    // Transaction interest estimated for the NFT payment
    // It is based as 10000=100%
    pub contract_interest: u64,
    //keeps track of the loan struct for a given loan ID
    pub loans_by_id: UnorderedMap<LoanId, Loan>,
    //keeps track of all the loan IDs for a given account
    pub loans_per_owner: LookupMap<AccountId, UnorderedSet<LoanId>>,
    //keeps track of all the loan IDs for a given account
    pub loans_per_lender: LookupMap<AccountId, UnorderedSet<LoanId>>,
    /// Total token amount payed.
    pub total_amount_payed: Balance,
    /// Total token amount lent.
    pub total_amount_lent: Balance,
    /// Duration of payment period for loans
    pub payment_period: u64,
    /// Fee payed to Nativo Loans
    pub contract_fee:u64, //200=2%
    /// If minting ntv is enabled
    pub is_minting_ntv: bool,
    ///multuplier for ntv tokens.
    pub ntv_multiply:u128,
    // loan current ath
    pub loan_current_ath: u128,
    /// loans active
    pub loans_active: u128,
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
            loans_by_id: UnorderedMap::new(StorageKey::LoansById.try_to_vec().unwrap()),
            loans_per_owner: LookupMap::new(StorageKey::LoansPerOwner.try_to_vec().unwrap()),
            loans_per_lender: LookupMap::new(StorageKey::LoansPerLender.try_to_vec().unwrap()),
            total_amount_payed: 0,
            total_amount_lent: 0,
            loan_current_ath: 0,
            loans_active: 0,
            ntv_multiply:3,
            payment_period:1_000_000_000 * 60 * 60 * 24 * 7,
            contract_fee, //200=2%
            is_minting_ntv: true,
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
        let id:LoanId = self.last_loan_id;
        let contract_id = env::predecessor_account_id();
        let signer_id = env::signer_account_id();
        let msg_json: MsgInput = from_str(&msg).unwrap();

        //calculate amount to be payed 
        let amount_to_loaner:u128 = u128::from(msg_json.loan_amount_requested)+(u128::from(msg_json.loan_amount_requested)*u128::from(self.contract_interest)/10000);
        env::log_str(&amount_to_loaner.to_string());

        let new_loan = Loan{
            nft_contract:contract_id,
            nft_id:token_id,
            nft_owner:signer_id.clone() ,
            description:msg_json.description,
            loan_requested:msg_json.loan_amount_requested,
            loan_payback:amount_to_loaner,
            status: LoanStatus::Pending,
            submission_time: env::block_timestamp(),
            loan_time:None,
            loan_deadline:None,
            loaner_id:None,
        };
        self.loans_by_id.insert(&id, &new_loan);
        self.internal_add_loan_to_owner(&signer_id, &id);
        self.last_loan_id += 1;
        /*env::log_str(
            &json!(new_loan)
            .to_string(),
        );*/
        
        //If for some reason the contract failed it need to returns the NFT to the original owner (true)
        return PromiseOrValue::Value(false);
    }

    pub fn minting_ntv(&mut self,enable:bool) -> String {
        self.is_the_owner();
        self.is_minting_ntv=enable;
        self.is_minting_ntv.to_string()
    }

    pub fn multiply_ntv(&mut self,multiply:u128) -> String {
        self.is_the_owner();
        self.ntv_multiply=multiply;
        self.ntv_multiply.to_string()
    }
    
    fn is_the_owner(&self){
        assert_eq!(self.owner_account_id,env::signer_account_id(),"you aren't the owner")
    }

    // Loan $NEAR Tokens to a loaning proposal
    #[payable]
    pub fn loan_for_nft(&mut self, loan_id: u64) -> Option<Loan> {
        //use a expect and explain that the loan wasnt found
        let mut loan:Loan = self.loans_by_id.get(&loan_id).expect("the token doesn't have an active loan");        
        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();
        self.total_amount_lent+=attached_deposit.clone();
        self.loan_current_ath=attached_deposit.clone();
        self.loans_active+=1;

        //Review that NFT is still available for loaning
        assert_eq!(LoanStatus::Pending,loan.status,"The NFT is not available for loaning");
        //Review that amount is the required
        assert_eq!(attached_deposit.clone(),loan.loan_requested,"The amount payed is not equal as the requested");
        //Review that loaner is not the same as NFT owner
        assert_ne!(signer_id.clone(),loan.nft_owner,"The owner cannot be the loaner");

        loan.status=LoanStatus::Loaned;
        loan.loaner_id = Some(signer_id.clone());
        loan.loan_time = Some(env::block_timestamp());
        //loan.loan_deadline = Some(env::block_timestamp()+60);
        loan.loan_deadline = Some(env::block_timestamp()+self.payment_period);

        let nft_owner = loan.nft_owner.clone();

        //here is pending of remove the 2% and be transfered to treasury

        //Here is removed % fee from amount transfered to owner
        let amount_to_owner:u128 = u128::from(loan.loan_requested)-(u128::from(loan.loan_requested)*u128::from(self.contract_fee)/10000);

        // Here is calculated the amount that will be sended to the treasury 
        let amount_to_treasury:u128 = u128::from(loan.loan_requested)*u128::from(self.contract_fee)/10000;

        //NTV Token payments
        if self.is_minting_ntv {
            //pay the NTV 
                let tokens_to_mint:u128 = u128::from(attached_deposit.clone()) * u128::from(self.ntv_multiply) ;
                // NTV for the buyer
                ext_nft::mint(
                    signer_id.clone(),
                    tokens_to_mint.to_string(),
                    NTVTOKEN_CONTRACT.to_string().try_into().unwrap(),
                    0000000000000000000000001,
                    10_000_000_000_000.into(),
                );
                env::log_str("the nvt token minting was payed");    
        }else{
            env::log_str("the nvt token minting is disabled");      
        }

        //Transfers are done
        Promise::new(nft_owner).transfer(amount_to_owner); //before the owner recived the amount for treasury
        Promise::new(self.treasury_account_id.clone()).transfer(amount_to_treasury);//before the treasury recived the amount for owner

        self.loans_by_id.insert(&loan_id, &loan);
        self.internal_add_loan_to_lender(&signer_id, &loan_id);
        return Some(loan);
    }

    #[payable]
    pub fn pay_loan(&mut self, loan_id: u64) -> Option<Loan> {
        let mut loan:Loan = self.loans_by_id.get(&loan_id).unwrap();
        let signer_id =env::signer_account_id();
        let attached_deposit=env::attached_deposit();
        let time_stamp=env::block_timestamp();

        self.total_amount_payed+=attached_deposit.clone();
        self.loans_active-=1;

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
        Promise::new(loan.loaner_id.clone().unwrap()).transfer(u128::from(attached_deposit.clone()));
        
        //NTV Token payments
        if self.is_minting_ntv {
            //pay the NTV 
                let tokens_to_mint = u128::from(attached_deposit.clone()) * u128::from(self.ntv_multiply) ;
                // NTV for the buyer
                ext_nft::mint(
                    signer_id.clone(),
                    tokens_to_mint.to_string(),
                    NTVTOKEN_CONTRACT.to_string().try_into().unwrap(),
                    0000000000000000000000001,
                    10_000_000_000_000.into(),
                );
                env::log_str("the nvt token minting was payed");    
        }else{
            env::log_str("the nvt token minting is disabled");      
        }

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
        self.loans_by_id.insert(&loan_id, &loan);

        return Some(loan);
    }

    //Canceled public offer for loaning
    #[payable]
    pub fn withdraw_nft_owner(&mut self, loan_id: u64){
        let mut loan:Loan = self.loans_by_id.get(&loan_id).unwrap();
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
        self.loans_by_id.insert(&loan_id, &loan);
        self.internal_remove_loan_from_owner(&signer_id, &loan_id);
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
    
    //If time has passed and the NFT owner didn't pay
    //The loaner can claim the NFT and transfer to their wallet
    #[payable]
    pub fn withdraw_nft_loaner(&mut self,loan_id:u64){
        let mut loan:Loan = self.loans_by_id.get(&loan_id).unwrap();
        let signer_id=env::signer_account_id();
        let time_stamp=env::block_timestamp();
        let deposit = env::attached_deposit();
        self.loans_active-=1;

        assert_eq!(time_stamp>=loan.loan_deadline.unwrap(),true,"The payment loan time has not expired");
        

        //assert!(loan.status!=LoanStatus::Loaned,"The NFT is under a loaning process.");

        //Review that claimer is the same as NFT loaner
        if signer_id != loan.loaner_id.clone().unwrap(){
            env::panic_str("You are not the loaner of this NFT");
        }

        loan.status=LoanStatus::Expired;
        self.loans_by_id.insert(&loan_id, &loan);
        self.internal_remove_loan_from_owner(&signer_id, &loan_id);
        self.internal_remove_loan_from_lender(&signer_id, &loan_id);
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