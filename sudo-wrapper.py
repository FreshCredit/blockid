import streamlit as st
from substrateinterface import SubstrateInterface, Keypair
from scalecodec.type_registry import load_type_registry_file
from scalecodec.base import ScaleBytes, ScaleDecoder

substrate = SubstrateInterface(
        url="ws://127.0.0.1:9944"  # URL of the Substrate node
  
    )

# Assuming a substrate instance is already initialized as `substrate`

st.sidebar.header('Sudo Functions')

# Load custom types for your chain
custom_type_registry = load_type_registry_file('<path to your custom type registry json>')

substrate.init_runtime_metadata(type_registry=custom_type_registry)

# Define the sudo account
sudo_keypair = Keypair.create_from_uri('//Alice') # replace with your sudo account's secret URI

module = st.sidebar.selectbox('Select a module', options=substrate.get_metadata_modules())
call = st.sidebar.selectbox('Select a call', options=substrate.get_module_calls(module))

# Depending on your needs, you might want to allow users to input parameters for the call
# Here is a simple example for a call without parameters
params = []

# The actual dispatch will vary depending on the module and call
# This is a generic example
sudo_call = {
    'call_module': 'Sudo',
    'call_function': 'sudo',
    'call_args': {
        'call': {
            'call_module': module,
            'call_function': call,
            'call_args': {}
        }
    }
}

if st.sidebar.button('Execute sudo call'):
    try:
        extrinsic = substrate.compose_call(
            call_module=sudo_call['call_module'],
            call_function=sudo_call['call_function'],
            call_params=sudo_call['call_args']
        )
        receipt = substrate.submit_extrinsic(extrinsic, keypair=sudo_keypair)
        st.success(f'Successfully executed sudo call, extrinsic hash: {receipt.extrinsic_hash}')
    except Exception as e:
        st.error(f'Failed to execute sudo call: {str(e)}')
