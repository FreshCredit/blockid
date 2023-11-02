import subprocess
import re
import threading
import queue
import streamlit as st

# Command to start the Polkadot node
command = "./polkadot/target/release/polkadot --alice --validator --base-path ./tmp/relay/alice --chain rococo-custom-3-raw.json --port 30333 --ws-port 9944"

# Define the regular expression pattern to search for the local node ID
pattern = re.compile(r"Local node identity is: (\S+)")

# Start the node process and capture its output
proc = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, shell=True)

# Create a queue to hold the output lines
q = queue.Queue()

# Function to capture the output of a running process
def capture_output(proc, q):
    for line in proc.stdout:
        q.put(line)
        match = pattern.search(line)
        if match:
            local_node_id = match.group(1)
            print("Local node identity (Parachain):", local_node_id)
            proc.terminate()


st.title("Initialization")

clicked = st.button("Initialize BlockID")
    # Read the output line by line until the local node ID is found
if clicked:
    # Start the thread to capture output
    t = threading.Thread(target=capture_output, args=(proc, q))
    t.start()
    
    while not q.empty():
        print("here")
        line = q.get()
        print(line, end='')
    # while proc.stdout.readline():
    #     line = proc.stdout.readline()
    #     if not line:
    #         break
    #     print(line, end='')  # Print the output for debugging purposes
    #     match = pattern.search(line)
    #     if match:
    #         local_node_id = match.group(1)
    #         print("Local node identity:", local_node_id)
    #         break

    # At this point, you have the local node ID stored in `local_node_id`
    # You can add code here to use it as needed
