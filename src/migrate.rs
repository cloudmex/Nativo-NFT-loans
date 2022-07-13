use crate::*;
use near_sdk::{Gas};

/// Gas for upgrading this contract on promise creation + deploying new contract.
pub const TGAS: u64 = 10_000_000_000_000;
pub const GAS_FOR_UPGRADE_SELF_DEPLOY: Gas = Gas(300_000_000_000_000);
pub const GAS_FOR_UPGRADE_REMOTE_DEPLOY: Gas = Gas(300_000_000_000_000);


#[near_bindgen]
impl NFTLoans {
    #[cfg(target_arch = "wasm32")]
    pub fn upgrade(self) {
        use near_sys as sys;
        // assert!(env::predecessor_account_id() == self.minter_account_id);
        //input is code:<Vec<u8> on REGISTER 0
        //log!("bytes.length {}", code.unwrap().len());
        const GAS_FOR_UPGRADE: u64 = 20 * TGAS; //gas occupied by this fn
       // const BLOCKCHAIN_INTERFACE_NOT_SET_ERR: &str = "Blockchain interface not set.";
        //after upgrade we call *pub fn migrate()* on the NEW CODE
        let current_id = env::current_account_id();
        let migrate_method_name = "migrate".as_bytes().to_vec();
        let attached_gas = env::prepaid_gas() - env::used_gas() - Gas(GAS_FOR_UPGRADE);
        unsafe {
            // Load input (new contract code) into register 0
            sys::input(0);

            //prepare self-call promise
            let promise_id =
                sys::promise_batch_create(current_id.as_bytes().len() as _, current_id.as_bytes().as_ptr() as _);

            //1st action, deploy/upgrade code (takes code from register 0)
            sys::promise_batch_action_deploy_contract(promise_id, u64::MAX as _, 0);

            // 2nd action, schedule a call to "migrate()".
            // Will execute on the **new code**
            sys::promise_batch_action_function_call(
                promise_id,
                migrate_method_name.len() as _,
                migrate_method_name.as_ptr() as _,
                0 as _,
                0 as _,
                0 as _,
                u64::from(attached_gas),
            );
        }
    }

/////////////////////METODO DE MIGRACIÖN
 
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_state: OldNFTLoans = env::state_read().expect("failed");
        
        env::log_str("old state readed");
        Self {
            owner_account_id: old_state.owner_account_id,
            treasury_account_id: old_state.treasury_account_id,
            last_loan_id: old_state.last_loan_id,
            contract_interest: old_state.contract_interest,
            loans_by_id: old_state.loans_by_id,
            loans_per_owner: old_state.loans_per_owner,
            loans_per_lender:old_state.loans_per_lender,
            total_amount_payed:old_state.total_amount_payed,
            total_amount_lent:old_state.total_amount_lent,
            loan_current_ath: 0,
            loans_active: 0,
            payment_period: old_state.payment_period,
            contract_fee:old_state.contract_fee,
            is_minting_ntv: old_state.is_minting_ntv,
            ntv_multiply: old_state.ntv_multiply,
        }
    }

}