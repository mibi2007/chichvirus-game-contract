export CONTRACT_ID=chichvirus-contract.mibi.testnet
export ACCOUNT_ID=mibi.testnet
near delete $CONTRACT_ID $ACCOUNT_ID
near create-account $CONTRACT_ID --masterAccount $ACCOUNT_ID --initialBalance 20