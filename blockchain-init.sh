#!/bin/bash

# Function to display help message
function display_help() {
    echo "Usage: $0 --parachain | --relaychain <chain_spec>"
    echo "You must specify either --parachain or --relaychain and provide the chainspec."
}

# Check if at least two arguments are provided
if [ $# -lt 2 ]; then
    display_help
    exit 1
fi

# Assign arguments to variables
MODE=$1
CHAIN_SPEC=$2



# Remove temporary files
rm -rf /tmp/relay
rm -rf /tmp/parachain

# Check mode and start appropriate nodes
if [ "$MODE" = "--relaychain" ]; then
    # Kill existing processes
    kill $(ps -u $(whoami) | grep ./target/release/polkadot | cut -w -f 3)
    # Start relay chain
    ./target/release/polkadot --alice --validator --base-path /tmp/relay/alice --chain "$CHAIN_SPEC" --port 30333 --ws-port 9944 &
    ./target/release/polkadot --bob --validator --base-path /tmp/relay/bob --chain "$CHAIN_SPEC" --port 30334 --ws-port 9945 &
    ./target/release/polkadot --charlie --validator --base-path /tmp/relay/charlie --chain "$CHAIN_SPEC" --port 30335 --ws-port 9946 &
    ./target/release/polkadot --dave --validator --base-path /tmp/relay/dave --chain "$CHAIN_SPEC" --port 30336 --ws-port 9947 &
    echo "Relay-Chain Started!!!"
elif [ "$MODE" = "--parachain" ]; then
    # Kill existing processes
    kill $(ps -u $(whoami) | grep ./target/release/parachain-blockid-node | cut -w -f 3)
    # Start parachain
    ./target/release/parachain-blockid-node --alice --collator --force-authoring --chain "$CHAIN_SPEC" --base-path /tmp/parachain/alice --port 40333 --ws-port 8844 -- --execution wasm --chain rococo-custom-3-raw.json --port 30343 --ws-port 9977 &
    ./target/release/parachain-blockid-node --bob --collator --force-authoring --chain "$CHAIN_SPEC" --base-path /tmp/parachain/bob --port 40334 --ws-port 8845 -- --execution wasm --chain rococo-custom-3-raw.json --port 30344 --ws-port 9978 &
    echo "Parachain Started!!!"
else
    # Invalid mode
    display_help
    exit 1
fi
