import argparse
import subprocess
import os

def display_help():
    """Display help message."""
    print("Usage: [--parachain | --relaychain] <chain_spec>")
    print("You must specify either --parachain or --relaychain and provide the chainspec.")

def kill_processes(process_name):
    """Kill processes matching the given name."""
    try:
        subprocess.run(["pkill", "-f", process_name], check=True)
    except subprocess.CalledProcessError:
        pass  # Handle the exception if needed

def start_relay_chain(chain_spec):
    """Start the relay chain."""
    subprocess.Popen(["./target/release/polkadot", "--alice", "--validator", "--base-path", "/tmp/relay/alice", "--chain", chain_spec, "--port", "30333", "--ws-port", "9944"])
    subprocess.Popen(["./target/release/polkadot", "--bob", "--validator", "--base-path", "/tmp/relay/bob", "--chain", chain_spec, "--port", "30334", "--ws-port", "9945"])
    subprocess.Popen(["./target/release/polkadot", "--charlie", "--validator", "--base-path", "/tmp/relay/charlie", "--chain", chain_spec, "--port", "30335", "--ws-port", "9946"])
    subprocess.Popen(["./target/release/polkadot", "--dave", "--validator", "--base-path", "/tmp/relay/dave", "--chain", chain_spec, "--port", "30336", "--ws-port", "9947"])
    # Repeat for other nodes (bob, charlie, dave)

def start_parachain(chain_spec):
    """Start the parachain."""
    subprocess.Popen(["./target/release/parachain-blockid-node", "--alice", "--collator", "--force-authoring", "--chain", chain_spec, "--base-path", "/tmp/parachain/alice", "--port", "40333", "--ws-port", "8844", "--rpc-cors", "all"])
    subprocess.Popen(["./target/release/parachain-blockid-node", "--bob", "--collator", "--force-authoring", "--chain", chain_spec, "--base-path", "/tmp/parachain/bob", "--port", "40334", "--ws-port", "8845"])
    subprocess.Popen(["./target/release/parachain-blockid-node", "--charlie", "--collator", "--force-authoring", "--chain", chain_spec, "--base-path", "/tmp/parachain/charlie", "--port", "40335", "--ws-port", "8846"])
    # Repeat for other nodes (bob)

def main():
    parser = argparse.ArgumentParser(description='Script to manage Polkadot nodes')
    parser.add_argument('--mode', required=True, choices=['parachain', 'relaychain'], help='Mode to run: parachain or relaychain')
    parser.add_argument('--chain_spec', required=True, help='Chain specification')
    args = parser.parse_args()

    # Remove temporary files
    os.system('rm -rf /tmp/relay')
    os.system('rm -rf /tmp/parachain')

    if args.mode == 'relaychain':
        kill_processes("./target/release/polkadot")
        start_relay_chain(args.chain_spec)
        print("Relay-Chain Started!!!")
    elif args.mode == 'parachain':
        kill_processes("./target/release/parachain-blockid-node")
        start_parachain(args.chain_spec)
        print("Parachain Started!!!")
    else:
        display_help()

if __name__ == "__main__":
    main()
