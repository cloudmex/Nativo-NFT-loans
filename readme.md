# P2P NFT loans

### NFT Transfer call - Paras Id
near call paras-token-v2.testnet nft_transfer_call '{"receiver_id": "dev-1647616622735-74951297117503","token_id": "299:9", "msg": "this is the message"}' --accountId alan_test.testnet --depositYocto 1  --gas 300000000000000

near view paras-token-v2.testnet nft_token '{"token_id":"299:9"}' 
### NFT Transfer call - Mintbase


near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "dev-1647616622735-74951297117503","token_id":"0", "msg": "this is the message"}' --accountId alan_test.testnet --depositYocto 1 --gas 300000000000000


near view alst77.mintspace2.testnet  nft_token '{"token_id":"0"}' 

### NFT Transfer call - Nativo NFT
near call nativo-minter.testnet nft_transfer_call '{"receiver_id": "dev-1647616622735-74951297117503","token_id": "24", "msg": "this is the message"}' --accountId alan_test.testnet --depositYocto 1 --gas 3000000000000

near call nativo-minter.testnet nft_transfer '{"receiver_id": "alan_test.testnet","token_id": "24", "msg": "this is the message"}' --accountId dev-1647616622735-74951297117503 --depositYocto 1

3000000000000
2428169188674


near call alst77.mintspace2.testnet nft_transfer_call '{"receiver_id": "alan_test.testnet","token_id":"0","msg": "this is the message"}' --accountId dev-1647616622735-74951297117503 --depositYocto 1 --gas 300000000000000