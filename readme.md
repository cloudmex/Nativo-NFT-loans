# Nativo NFT - P2P loans

![Logo](https://v2.nativonft.app/static/media/nativologocrop.15afa4d2.png)

NFT loans allows you to have access to liquidity without loosing the ownership of your NFT's
1. Secure your NFT in NFT loans and request an amount of tokens
2. People loan you the amount of tokens you expect to receive
3. You have the option to payback the tokens + interest or to give NFT to the loaner

### Initializing the contract
CONTRACT_ID=nativoloans.testnet
near call $CONTRACT_ID new '{"owner_account_id": "nativoloans.testnet","treasury_account_id": "nativoloans.testnet","contract_interest": 800,"contract_fee": 200  }' --accountId nativoloans.testnet 

### Viewing all the loans paginated
near view $CONTRACT_ID get_nfts_for_loan '{"from_index":"0","limit":50}'

### View last loan
near view $CONTRACT_ID get_last_loan

### Loan NEARS in exchange of an NFT or APY
near call $CONTRACT_ID loan_for_nft '{"loan_id":1}' --accountId darkjoehank.testnet --deposit 5

### Pay a loan you received + interes rate (8%)
near call $CONTRACT_ID pay_loan '{"loan_id":1}' --accountId joehank.testnet --deposit 100

### Cancel your loan and recover your NFT
near call $CONTRACT_ID withdraw_nft_owner ‘{“loan_id”:1}’ --accountId joeahank.testnet --depositYocto 1 --gas 100000000000000

### If the time to pay the loan has already expired, the lender can claim the token
near call $CONTRACT_ID withdraw_nft_loaner ‘{“loan_id”:15}’ --accountId joehank.testnet --depositYocto 1 --gas 100000000000000

### Get the metrics of the loans
near view $CONTRACT_ID get_loans_metrics

### Loans supply for owner method
near call $CONTRACT_ID loan_supply_for_owner '{"account_id":"joehank.testnet"}' --accountId darkjoehank.testnet

### Loans pagination for owner
near call $CONTRACT_ID loans_for_owner '{"account_id":"joehank.testnet"}' --accountId darkjoehank.testnet

### Loans supply for lender method
near call $CONTRACT_ID loan_supply_for_lender '{"account_id":"darkjoehank.testnet"}' --accountId darkjoehank.testnet

### Loans pagination for lender
near call $CONTRACT_ID loans_for_lender '{"account_id":"darkjoehank.testnet"}' --accountId darkjoehank.testnet

### Ask for a loaning - Mintbase
near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1648670267690-23487881027419","token_id":"0", "msg": "{\"description\": \"list a new nft for loaning\", \"loan_amount_requested\": 100000000000000000000000000 }"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000


near call alst77.mintspace2.testnet nft_transfer '{"receiver_id": "alan_test.testnet","token_id":"0","msg":""}' --accountId $CONTRACT_ID --depositYocto 1 
near view alst77.mintspace2.testnet  nft_token '{"token_id":"0"}' 

### Ask for a loaning - Paras Id
near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "nativoloans.testnet","token_id":"1445:1", "msg": "{\"description\": \"list a new nft for loaning\", \"loan_amount_requested\": 1000000000000000000000000 }"}' --accountId joehank.testnet --depositYocto 1 --gas 100000000000000

near view paras-token-v2.testnet nft_token '{"token_id":"299:9"}' 

### Ask for a loaning - Nativo NFT
near call minterv2.nativo-minter.testnet nft_transfer_call '{"receiver_id": "nativoloans.testnet","token_id":"73", "msg": "{\"description\": \"list a new nft for loaning\", \"loan_amount_requested\": 1000000000000000000000000 }"}' --accountId joehank.testnet --depositYocto 1 --gas 100000000000000

