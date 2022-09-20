#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
# near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/.wasm
# near deploy --wasmFile out/contract.wasm --accountId $CONTRACT_ID --initFunction new '{"owner_id": $ACCOUNT_ID}'
near deploy --wasmFile out/contract.wasm --accountId $CONTRACT_ID --initFunction new --initArgs '{"owner_id": "mibi.testnet"}'