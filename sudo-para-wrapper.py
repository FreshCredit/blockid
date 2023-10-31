import streamlit as st
from substrateinterface import SubstrateInterface, Keypair
import time
import threading
import sys

# Global variable to control the background thread
stop_thread = False

def listen_for_new_blocks(substrate):
    for header in substrate.subscribe_block_headers():
        if stop_thread:
            break

        block_hash = header.get('parentHash')
        events = substrate.get_block_events(block_hash)

        # print or do something with events
        for event in events:
            print(event)

        # sleep for a short period to allow other threads to run
        time.sleep(0.1)




def connect_to_substrate_node():
    substrate = SubstrateInterface(
        url="ws://127.0.0.1:9944"  # URL of the Substrate node
  
    )
    try:
        chain_head = substrate.get_chain_head()
        if chain_head:
            st.write("Chain Head: " + chain_head)
            st.success("Successfully connected to the Substrate node.")
            return substrate
        else:
            st.error("Failed to fetch chain head. The node might not be running or there is a network issue.")
            return False
    except Exception as e:
        st.error(f"An error occurred: {str(e)}")
        return False




def submit_transaction(selected_module):
    try:
        substrate = SubstrateInterface(
        url="ws://127.0.0.1:9944"  # URL of the Substrate node
    )
    except NameError:
        if not connect_to_substrate_node():
            return "Connection to Substrate node failed"

    # The keypair of the account that has the sudo key
    sudo_keypair = Keypair.create_from_uri('//Alice')

    # The genesis state and validation code for your parachain
    with open('para-2000-genesis-state', 'rb') as f:
        genesis_head = f.read().hex()
    with open('para-2000-wasm', 'rb') as f:
        validation_code = f.read().hex()

    # The call you want to execute as sudo
    call = substrate.compose_call(
        call_module=selected_module, 
        call_function='sudo_schedule_para_initialize',  
        call_params={
            'id': 2000,  # the id of your parachain
            'genesis': {
                'genesis_head': genesis_head,
                'validation_code': validation_code,
                'parachain': False
            }
        }
    )

    # The sudo call that wraps the original call
    sudo_call = substrate.compose_call(
        call_module='Sudo', 
        call_function='sudo', 
        call_params={
            'call': call
        }
    )

    # Create, sign, and submit the extrinsic
    extrinsic = substrate.create_signed_extrinsic(call=sudo_call, keypair=sudo_keypair)
    result = substrate.submit_extrinsic(extrinsic, wait_for_inclusion=True)

    return result


# Streamlit app starts here

st.title('Parachain Registration')

substrate = SubstrateInterface(url="ws://127.0.0.1:9944")  # URL of the Substrate node
modules = substrate.get_metadata_modules()
module_names = []

for index, module in enumerate(modules):
    module_names.append(module["name"])

if st.button('Check Connection'):
    substrate = connect_to_substrate_node()
    print(substrate)

selected_module = st.selectbox("select a module", options=module_names)

if st.button('Submit Transaction'):
    result = submit_transaction(selected_module)
    st.write(result)
