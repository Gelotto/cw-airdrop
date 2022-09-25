#!/bin/bash

CMD=$1
NETWORK=$2
CONTRACT_ADDR=$(cat $3)
NODE=
CHAIN_ID=
FLAGS=

shift 3

case $NETWORK in
  testnet)
    NODE="https://rpc.uni.juno.deuslabs.fi:443"
    CHAIN_ID=uni-3
    DENOM=ujunox
    ;;
  mainnet)
    NODE="https://rpc-juno.itastakers.com",
    CHAIN_ID=juno-1
    DENOM=ujuno
    ;;
  devnet)
    NODE="http://localhost:26657"
    CHAIN_ID=testing
    DENOM=ujunox
    ;;
esac

increment_claim() {
  sender=$1
  claimant=$2
  amount=$3
  expiry=$4
  msg='{"increment_claim":{"claimant":"'$claimant'","amount":"'$amount'","expiry":'$expiry'}}'
  flags="\
  --node $NODE \
  --gas-prices 0.025$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.5 \
  --broadcast-mode block \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "$msg" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" "$msg" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}


increment_claim_batch() {
  sender=$1
  claimant=$2
  amount=$3
  expiry=$4
  msg='{"increment_claim_batch":{"claims":[{"claimant":"'$claimant'","amount":"'$amount'","expiry":'$expiry'},{"claimant":"'$claimant'","amount":"'$amount'","expiry":'$expiry'}]}}'
  flags="\
  --node $NODE \
  --gas-prices 0.025$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.5 \
  --broadcast-mode block \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "$msg" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" "$msg" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

claim() {
  sender=$1
  claimant=$2
  amount=$3
  if [ -n "$amount" ]; then
    msg='{"claim":{"claimant":"'$claimant'", "amount":"'$amount'"}}'
  else 
    msg='{"claim":{"claimant":"'$claimant'"}}'
  fi
  flags="\
  --node $NODE \
  --gas-prices 0.025$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.5 \
  --broadcast-mode block \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "$msg" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" "$msg" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

withdraw_claim() {
  sender=$1
  claimant=$2
  msg='{"withdraw_claim":{"claimant":"'$claimant'"}}'
  flags="\
  --node $NODE \
  --gas-prices 0.025$DENOM \
  --chain-id $CHAIN_ID \
  --from $sender \
  --gas auto \
  --gas-adjustment 1.5 \
  --broadcast-mode block \
  --output json \
  -y \
  "
  echo junod tx wasm execute $CONTRACT_ADDR "$msg" "$flags"
  response=$(junod tx wasm execute "$CONTRACT_ADDR" "$msg" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}



get_claim_amount() {
  claimant=$1
  query='{"get_claim_amount":{"claimant":"'$claimant'"}}'
  flags="--chain-id $CHAIN_ID --output json --node $NODE"
  echo junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags
  response=$(junod query wasm contract-state smart $CONTRACT_ADDR "$query" $flags)
  echo $response | ./bin/utils/base64-decode-attributes | jq
}

set -e

case $CMD in
  increment-claim)
    increment_claim $*;;
  increment-claim-batch)
    increment_claim_batch $*;;
  withdraw-claim)
    withdraw_claim $*;;
  claim)
    claim $*;;
  get-claim-amount) 
    get_claim_amount $*;;
esac