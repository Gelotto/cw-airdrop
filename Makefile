network ?= devnet  # network := devnet|mainnet|testnet
contract_addr_filepath ?= $(release_dirpath)/contract_addr.txt
wasm_filename ?= cw_contract_template.wasm
release_dirpath ?= ./release
sender ?= juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
label ?= airdrop

# build optimized WASM artifact
build:
	./bin/build

# deploy WASM file (generated from `make build`)
deploy:
	./bin/deploy ./artifacts/$(wasm_filename) $(network) $(sender)

# instantiate last contract to be deployed using code ID in release dir code-id file
instantiate:
	./bin/instantiate $(network) $(sender) $(label) $(cw20_token_address)

# run all unit tests
test:
	RUST_BACKTRACE=1 cargo unit-test

# Generate the contract's JSONSchema JSON files in schemas/
schemas:
	cargo schema

# Run/start local "devnet" validator docker image	
validator:
	./bin/validator

increment-claim:
	./client.sh increment-claim $(network) $(contract_addr_filepath) $(sender) $(claimant) $(amount) null

increment-claim-batch:
	./client.sh increment-claim-batch $(network) $(contract_addr_filepath) $(sender) $(claimant) $(amount) null

claim:
	./client.sh claim $(network) $(contract_addr_filepath) $(sender) $(claimant) $(amount)

withdraw-claim:
	./client.sh withdraw-claim $(network) $(contract_addr_filepath) $(sender) $(claimant) $(amount)

get-claim-amount:
	./client.sh get-claim-amount $(network) $(contract_addr_filepath) $(claimant)