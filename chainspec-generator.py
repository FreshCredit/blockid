import streamlit as st
from string import Template
import subprocess
import json
import re

# Define templates for commands
cmd_templates = {
    "validator_authority": "./substrate/target/release/subkey inspect --scheme sr25519 --network substrate //${account}//stash",
    "grandpa_session": "./substrate/target/release/subkey inspect --scheme ed25519 --network substrate //${account}",
    "polkadot_address": "./substrate/target/release/subkey inspect --scheme sr25519 --network substrate //${account}",
    "encoded_BEEFY": "./substrate/target/release/subkey inspect --scheme ecdsa --network substrate //${account}",
}

def run_command(cmd):
    result = subprocess.run(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        st.error(f"Command failed with error {result.stderr}")
        return None
    else:
        ss58_key_pattern = r"Public key \(SS58\):\s+(\w+)"
        match = re.search(ss58_key_pattern, result.stdout)
        return match.group(1) if match else None

# Title
st.header("Chainspec Generator")

# Capture user input
accounts = st.text_input("Enter the account names (comma separated):")

if accounts:
    accounts_list = [account.strip() for account in accounts.split(",")]

    st.header("Generated JSON")

    final_keys = []
    # For each account, generate and run the commands, collect results
    for account in accounts_list:
        keys = {}
        for key, cmd_template in cmd_templates.items():
            cmd = Template(cmd_template).substitute(account=account)
            output = run_command(cmd)
            if output:
                keys[key] = output.strip()

        # Construct the account keys
        account_keys = [
            keys["validator_authority"],
            keys["validator_authority"],
            {
                "grandpa": keys["grandpa_session"],
                "babe": keys["polkadot_address"],
                "im_online": keys["polkadot_address"],
                "para_validator": keys["polkadot_address"],
                "para_assignment": keys["polkadot_address"],
                "authority_discovery": keys["polkadot_address"],
                "beefy": keys["encoded_BEEFY"],
            }
        ]

        final_keys.append(account_keys)

    # Construct the final JSON
    final_json = {
        "keys": final_keys
    }

    # Display the final JSON
    st.text(json.dumps(final_json, indent=2))
