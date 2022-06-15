use crate::*;
use near_sdk::{CryptoHash};
use std::mem::size_of;

//used to generate a unique prefix in our storage collections (this is to avoid data collisions)
pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    //get the default hash
    let mut hash = CryptoHash::default();
    //we hash the account ID and return it
    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}

impl NFTLoans {
    //add a loan to the set of tokens an owner has
    pub(crate) fn internal_add_loan_to_owner(
        &mut self,
        account_id: &AccountId,
        loan_id: &LoanId,
    ) {
        //get the set of tokens for the given account
        let mut loans_set = self.loans_per_owner.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::LoanPerOwnerInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        loans_set.insert(loan_id);

        //we insert that set for the given account ID. 
        self.loans_per_owner.insert(account_id, &loans_set);
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_loan_from_owner(
        &mut self,
        account_id: &AccountId,
        loan_id: &LoanId,
    ) {
        //we get the set of tokens that the owner has
        let mut loans_set = self
            .loans_per_owner
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("Loan should be owned by the sender");

        //we remove the the token_id from the set of tokens
        loans_set.remove(loan_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if loans_set.is_empty() {
            self.loans_per_owner.remove(account_id);
        } else {
        //if the token set is not empty, we simply insert it back for the account ID. 
            self.loans_per_owner.insert(account_id, &loans_set);
        }
    }

    pub(crate) fn internal_add_loan_to_lender(
        &mut self,
        account_id: &AccountId,
        loan_id: &LoanId,
    ) {
        //get the set of tokens for the given account
        let mut loans_set = self.loans_per_lender.get(account_id).unwrap_or_else(|| {
            //if the account doesn't have any tokens, we create a new unordered set
            UnorderedSet::new(
                StorageKey::LoanPerLenderInner {
                    //we get a new unique prefix for the collection
                    account_id_hash: hash_account_id(&account_id),
                }
                .try_to_vec()
                .unwrap(),
            )
        });

        //we insert the token ID into the set
        loans_set.insert(loan_id);

        //we insert that set for the given account ID. 
        self.loans_per_lender.insert(account_id, &loans_set);
    }

    //remove a token from an owner (internal method and can't be called directly via CLI).
    pub(crate) fn internal_remove_loan_from_lender(
        &mut self,
        account_id: &AccountId,
        loan_id: &LoanId,
    ) {
        //we get the set of tokens that the owner has
        let mut loans_set = self
            .loans_per_lender
            .get(account_id)
            //if there is no set of tokens for the owner, we panic with the following message:
            .expect("Loan should be lended by the sender");

        //we remove the the token_id from the set of tokens
        loans_set.remove(loan_id);

        //if the token set is now empty, we remove the owner from the tokens_per_owner collection
        if loans_set.is_empty() {
            self.loans_per_lender.remove(account_id);
        } else {
        //if the token set is not empty, we simply insert it back for the account ID. 
            self.loans_per_lender.insert(account_id, &loans_set);
        }
    }
}