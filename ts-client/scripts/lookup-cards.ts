const { ApiPromise, WsProvider } = require("@polkadot/api");

const endpoint = "ws://127.0.0.1:8844";
const blockHash = "";
async function lookupCards() {
	// Instantiate the API
	const wsProvider = new WsProvider(endpoint);
	const api = await ApiPromise.create({ provider: wsProvider });
	console.log(`Connect to node ${endpoint}... Successful!`);
	const signedBlock = await api.rpc.chain.getBlock(blockHash);

	// get the api and events at a specific block
	const apiAt = await api.at(signedBlock.block.header.hash);
	const allRecords = await apiAt.query.system.events();
	const { parentHash } = await api.rpc.chain.getHeader(blockHash);

	// map between the extrinsics and events
    signedBlock.block.extrinsics.forEach(({ method: { method, section } } : any, index : any) => {
		// filter the specific events based on the phase and then the
		// index of our extrinsic in the block
		allRecords
			.filter(({ phase }: any) => phase.isApplyExtrinsic && phase.asApplyExtrinsic.eq(index))
			// test the events against the specific types we are looking for
			.forEach(({ event }: any) => {
				if (event.section === "nftPallet" && event.method === "NFTMinted") {
					// Assuming the offchain data is the first item in the event's data array
					const offchainData = event.data[2];
					if (offchainData) {
						console.log("Parent Hash: ", parentHash.toHex());
						console.log("Block Hash: ", blockHash); // Need block hash for retrievel
						console.log("Offchain Data: ", offchainData.toString()); // Convert to string or appropriate type
						process.exit();
					}
				}
			});
	});
}

lookupCards()
	.catch(console.error)
	.finally(() => process.exit());
