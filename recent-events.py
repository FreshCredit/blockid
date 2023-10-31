import streamlit as st
from substrateinterface import SubstrateInterface
import threading
import queue
import pandas as pd
import pprint

st.set_page_config(layout="wide")


# Global variable to control the background thread
stop_thread = True

# Maximum number of recent blocks to display
MAX_RECENT_BLOCKS = 30
MAX_RECENT_EVENTS = 30

# Queue to store recent blocks
recent_blocks = queue.Queue(MAX_RECENT_BLOCKS)
recent_events = queue.Queue(MAX_RECENT_EVENTS)

def block_subscription_handler(obj, update_nr, subscription_id):
    #print(f"#{obj['header']['number']} produced by {obj['header']}")
    recent_blocks.put([obj['header']['number'], obj['header']['parentHash']])
   
def event_subscription_handler(obj, update_nr, subscription_id):
    # print(f"#{obj['header']['number']} produced by {obj['header']}")
    number = str(obj['header']['number'])   
    hash = str(obj['header']['parentHash'])
    events = substrate.get_events(block_hash=hash)
    recent_events.put([number, str(events)])
   
def listen_for_new_blocks(stop_thread):
    while stop_thread:
        substrate.subscribe_block_headers(subscription_handler=block_subscription_handler, include_author=True, ignore_decoding_errors=True)

def listen_for_new_events(stop_thread):
    while stop_thread:
        substrate.subscribe_block_headers(subscription_handler=event_subscription_handler, include_author=True, ignore_decoding_errors=True)

col1, col2  = st.columns(2)


with col1:
    # Placeholder to display recent blocks
    block_placeholder=st.empty()     

with col2:
    # Placeholder to display recent events
    event_placeholder=st.empty()    
        
def handle_block_updates(block_value):
        df = pd.DataFrame(block_value, columns=['#', 'Recent Blocks'])
        sdf = df.sort_values(by='#', ascending=False)
        block_placeholder.dataframe(sdf.set_index(sdf.columns[0]))

def handle_event_updates(event_value):
        df = pd.DataFrame(event_value, columns=['#', 'Recent Events'])
        sdf = df.sort_values(by='#', ascending=False)
        event_placeholder.dataframe(sdf.set_index(sdf.columns[0]))

with st.sidebar:
    button_placeholder = st.empty()

    sbutton_placeholder = st.empty()

start_button = sbutton_placeholder.button("Start listening")

stop_button = button_placeholder.button('Stop listening')


def main():
    global stop_thread
    # Check for updates while the worker thread is running
    block_values = []
    event_values = []
    timeout = 10    

    # Process updates from blocks queue
    blocks_empty = True
    while blocks_empty:
        try:
            block_values.append(recent_blocks.get(block=True, timeout=timeout))
            # event_values.append(recent_events.get(block=True, timeout=timeout))
            handle_block_updates(block_values)
            # handle_event_updates(event_values)
            if stop_button:
                stop_thread = False
        except queue.Empty:
            blocks_empty = False
            block_thread.join()  # Wait for the background thread to finish
            # event_thread.join()  # Wait for the background thread to finish

if __name__ == '__main__':
    stop_thread = True
    substrate = SubstrateInterface(url="ws://127.0.0.1:9944",ss58_format=42,
        type_registry_preset='polkadot')
    # substrate.subscribe_block_headers()
    # Start background thread
    if start_button:
        block_thread = threading.Thread(target=listen_for_new_blocks, args=(stop_thread,))
        block_thread.start()
        event_thread = threading.Thread(target=listen_for_new_events, args=(stop_thread,))
        event_thread.start()
        main()
