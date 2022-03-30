# Nativo NFT - P2P loans

![Logo](https://v2.nativonft.app/static/media/nativologocrop.15afa4d2.png)

NFT loans allows you to have access to liquidity without loosing the ownership of your NFTS
1.- Secure your NFT in NFT loans and request an amount of tokens
2.- People loan you the amount of tokens you expect to receive
3.- You have the option to payback the tokens + interest or to give NFT to the loaner

### Initializing the contract
CONTRACT_ID="dev-1648599492191-52724814339002"
near call $CONTRACT_ID new '{"owner_account_id": "dev-1648590312447-45249611681386","treasury_account_id": "dev-1648590312447-45249611681386","contract_apy": 80 }' --accountId alan_test.testnet 

### Viewing all the loans paginated
near view $CONTRACT_ID get_nfts_for_loan '{"from_index":0,"limit":50}'

### View last loan
near view $CONTRACT_ID get_last_loan

### Loan NEARS in exchange of an NFT or APY
near call $CONTRACT_ID loan_for_nft '{"loan_id":0}' --accountId hack_test.testnet --depositYocto 100

### Ask for a loaning - Paras Id
near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "dev-1647921766612-74437195022952","token_id": "299:9", "msg": "{\"description\": \"list my nft for loaning\", \"loan_requested\": \"100\"}"}' --accountId alan_test.testnet --depositYocto 1  --gas 300000000000000

near view paras-token-v2.testnet nft_token '{"token_id":"299:9"}' 
### Ask for a loaning - Mintbase

near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1648590312447-45249611681386","token_id":"0", "msg": "{\"description\": \"list\", \"loan_amount_requested\": 100 }"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000


near call alst77.mintspace2.testnet nft_transfer '{"receiver_id": "alan_test.testnet","token_id":"0","msg":""}' --accountId dev-1648590312447-45249611681386 --depositYocto 1 
near view alst77.mintspace2.testnet  nft_token '{"token_id":"0"}' 

### Ask for a loaning - Nativo NFT
near call nativo-minter.testnet nft_transfer_call '{"receiver_id": "dev-1647921766612-74437195022952","token_id": "39", "msg": "{\"description\": \"list\", \"loan_requested\": \"100\"}"}' --accountId alan_test.testnet --depositYocto 1 --gas 3000000000000

near call nativo-minter.testnet nft_transfer '{"receiver_id": "alan_test.testnet","token_id": "24", "msg": "this is the message"}' --accountId dev-1647616622735-74951297117503 --depositYocto 1

near view nativo-minter.testnet nft_token '{"token_id":"24"}' 
3000000000000
2428169188674


near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "alan_test.testnet","token_id":"0","msg": "this is the message"}' --accountId dev-1647616622735-74951297117503 --depositYocto 1 --gas 300000000000000