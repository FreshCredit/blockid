// import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
// import { connect, sendTransactionAsync } from "../src/utils";

// import "../src/interfaces/augment-api";

// (async () => {
//   let { api, alice } = await connect();

//   const credits = await api.query.nftPallet.nfts.entries();
//   for (const entry of credits) {
//     if (entry[1].isSome) {
//       const credit = entry[1].unwrap();
//       console.log(`hahahahahahahahahaah ${credit.name.toUtf8()} ${credit}`);
//       const id = entry[0].args[0];
//       if (!credit.cardsMinted.toHuman()) {
//         const tx = api.tx.nft.mintNft(id);
//         await sendTransactionAsync(api, alice, tx, `mint cards for ${credit.name.toUtf8()}`);
//       } else {
//         console.log(`skipping ${credit.name.toUtf8()} because they already have cards minted`);
//       }
//     }
//   }
// })()


// Import the API, Keyring and some utility functions

const {ApiPromise, WsProvider} = require('@polkadot/api');
const {blake2AsHex} = require('@polkadot/util-crypto');
const {Keyring} = require('@polkadot/keyring');

function getRandomInt(min: number, max: number): number {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}


let data = {
    id: "1",
    fname: "John",
    lname: "Doe",
    email: "johndoe3@mail.com",
    mobile: `${getRandomInt(1111111111,9999999999)}`,
    photo: "",
    ssn: "",
    dob: "",
    street1: "",
    street2: "",
    city: "",
    state: "",
    zip: ""
  }

  const endpoint = 'ws://127.0.0.1:8844';
  const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
  
  async function main() {
    // Instantiate the API
    const wsProvider = new WsProvider(endpoint);
    const api = await ApiPromise.create({provider: wsProvider});
    console.log(`Connect to node ${endpoint}... Successful!`);
    // simple query function
    // let total_nft = await api.query.nftPallet.NextNFTId();
    // console.log("total nft ", total_nft);
    let dataObject = JSON.stringify(data, null, 2); // spacing level = 2
    console.log(`Data being hashed:`, '\n', dataObject)
    const hash = blake2AsHex(data);
    console.log("\nHashing... Successful!\n\nStoring on user device only!!!\n")
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    console.log(`Using Alice keyring ${JSON.stringify(alice.address)} to mint ${hash}\n`)
    // Sign and send the transaction using our account
    const transfer = api.tx.nftPallet.mintNft(hash);
    const out = await transfer.signAndSend(alice);
    console.log('Transfer sent with hash', [out.toHex()]);

    // Call the event monitoring function
    await lookupTransaction(api);
}

async function lookupTransaction(api:any) {
  while(true){  
    try {
     // no blockHash is specified, so we retrieve the latest
    const signedBlock = await api.rpc.chain.getBlock();

    // get the api and events at a specific block
    const apiAt = await api.at(signedBlock.block.header.hash);
    const allRecords = await apiAt.query.system.events();
    const { hash, parentHash } = await api.rpc.chain.getHeader();

    // map between the extrinsics and events
    signedBlock.block.extrinsics.forEach(({ method: { method, section } } : any, index : any) => {
      
        // filter the specific events based on the phase and then the
        // index of our extrinsic in the block
        const events = allRecords
      .filter(({ phase } : any) =>
        phase.isApplyExtrinsic &&
        phase.asApplyExtrinsic.eq(index)
      )
      .map(({ event }: any) => `${event.section}.${event.method}`);
  allRecords
        .filter(({ phase } : any) =>
          phase.isApplyExtrinsic &&
          phase.asApplyExtrinsic.eq(index)
        )
        // test the events against the specific types we are looking for
        .forEach(({ event } : any) => {
          if (event.section === 'nftPallet' && event.method === 'NFTMinted') {
            // Assuming the offchain data is the first item in the event's data array
            const offchainData = event.data[2];
            if (offchainData) {
              console.log('Parent Hash: ', parentHash.toHex())
              console.log('Block Hash: ', hash.toHex())  // Need block hash for retrievel
              console.log('Offchain Data: ', offchainData.toString()); // Convert to string or appropriate type
              process.exit()
            }
          }
          if (api.events.system.ExtrinsicSuccess.is(event)) {
            // extract the data for this event
            // (In TS, because of the guard above, these will be typed)

            console.log("Waiting on event...")
          } else if (api.events.system.ExtrinsicFailed.is(event)) {
            // extract the data for this event
            const [dispatchError] = event.data;
            let errorInfo;

            // decode the error
            if (dispatchError.isModule) {
              // for module errors, we have the section indexed, lookup
              // (For specific known errors, we can also do a check against the
              // api.errors.<module>.<ErrorName>.is(dispatchError.asModule) guard)
              const decoded = api.registry.findMetaError(dispatchError.asModule);

              errorInfo = `${decoded.section}.${decoded.name}`;
            } else {
              // Other, CannotLookup, BadOrigin, no extra info
              errorInfo = dispatchError.toString();
            }
            console.log(`${section}.${method}:: ExtrinsicFailed:: ${errorInfo}`);
          }
        });
    });
      
    } catch (error) {
      console.log(error)
    }
    await new Promise((resolve) => setTimeout(resolve, 5000));
  }
}

main().catch(console.error).finally(() => process.exit());