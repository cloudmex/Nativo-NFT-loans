use crate::*;

#[near_bindgen]
impl NFTLoans {

    //get the information for a specific token ID
    fn get_nft_loan(&self, loan_id: LoanId) -> Option<LoanOutput> {
        //if there is some loan ID in the loans_by_id collection
        if let loans = self.loans_by_id.get(&loan_id).unwrap() {
            //we'll return the data for that loan
            Some(LoanOutput {
                id:loan_id,
                loan:loans.into(),
            })
        } else { //if there wasn't a loan ID in the loans_by_id collection, we return None
            None
        }
    }

    // pub fn get_nft_loan(&self, loan_id: u64) -> LoanOutput {
    //     let loans = self.loans.get(&loan_id).expect("ERR_NO_LOAN");
    //     LoanOutput {
    //         id:loan_id,
    //         loan: loans.into(),
    //     }
    // }

    //Query for nft tokens on the contract regardless of the owner using pagination
    pub fn get_nfts_for_loan(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<LoanOutput> {
        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through each token using an iterator
        self.loans_by_id.keys_as_vector().iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|loan_id| self.get_nft_loan(loan_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    //View wich NFT are available for loaning
    // pub fn get_nfts_for_loan(&self, from_index: u64, limit: u64)-> Vec<LoanOutput> {
    //     (from_index..min(self.last_loan_id, from_index + limit))
    //         .filter_map(|id| {
    //             self.loans.get(&id).map(|loan| LoanOutput {
    //                 id,
    //                 loan: loan.into(),
    //             })
    //         })
    //         .collect()
    // }

    //View the loan_id of the last loan
    pub fn get_contract_interest(&self)-> u64 {
        self.contract_interest
    }
    
    //View the loan_id of the last loan
    pub fn get_last_loan(&self)-> u64 {
        self.last_loan_id
    }

    //get the total supply of NFTs for a given owner
    pub fn loan_supply_for_owner(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let loans_for_owner_set = self.loans_per_owner.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(loans_for_owner_set) = loans_for_owner_set {
            U128(loans_for_owner_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn loans_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<LoanOutput> {
        //get the set of tokens for the passed in owner
        let loans_for_owner_set = self.loans_per_owner.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let loans = if let Some(loans_for_owner_set) = loans_for_owner_set {
            loans_for_owner_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        loans.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|loan_id| self.get_nft_loan(loan_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }


    //get the total supply of NFTs for a given owner
    pub fn loan_supply_for_lender(
        &self,
        account_id: AccountId,
    ) -> U128 {
        //get the set of tokens for the passed in owner
        let loans_for_lender_set = self.loans_per_lender.get(&account_id);

        //if there is some set of tokens, we'll return the length as a U128
        if let Some(loans_for_lender_set) = loans_for_lender_set {
            U128(loans_for_lender_set.len() as u128)
        } else {
            //if there isn't a set of tokens for the passed in account ID, we'll return 0
            U128(0)
        }
    }

    //Query for all the tokens for an owner
    pub fn loans_for_lender(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<LoanOutput> {
        //get the set of tokens for the passed in owner
        let loans_for_lender_set = self.loans_per_lender.get(&account_id);
        //if there is some set of tokens, we'll set the tokens variable equal to that set
        let loans = if let Some(loans_for_lender_set) = loans_for_lender_set {
            loans_for_lender_set
        } else {
            //if there is no set of tokens, we'll simply return an empty vector. 
            return vec![];
        };

        //where to start pagination - if we have a from_index, we'll use that - otherwise start from 0 index
        let start = u128::from(from_index.unwrap_or(U128(0)));

        //iterate through the keys vector
        loans.iter()
            //skip to the index we specified in the start variable
            .skip(start as usize) 
            //take the first "limit" elements in the vector. If we didn't specify a limit, use 50
            .take(limit.unwrap_or(50) as usize) 
            //we'll map the token IDs which are strings into Json Tokens
            .map(|loan_id| self.get_nft_loan(loan_id.clone()).unwrap())
            //since we turned the keys into an iterator, we need to turn it back into a vector to return
            .collect()
    }

    pub fn get_loans_metrics(& self) -> Metrics {
        let metrics = Metrics {
            
            total_loans: self.last_loan_id,
            
            total_amount_payed: self.total_amount_payed,
            
            ntv_status:self.is_minting_ntv,

            ntv_multiply: self.ntv_multiply,
            
            total_loans_active: self.loans_active,
            
            total_amount_lent: self.total_amount_lent,
            
            loan_current_ath: self.loan_current_ath,
       };
       metrics
    }
   
    pub fn is_ntv_enable(&self)->bool {
        self.is_minting_ntv
    }
}