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

import {WsProvider} from "@polkadot/api";

const {ApiPromise} = require('@polkadot/api');
const {blake2AsHex} = require('@polkadot/util-crypto');
const {Keyring} = require('@polkadot/keyring');


const hash = blake2AsHex("Just some nft text")
const ALICE = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";

async function main() {
    // Instantiate the API
    const wsProvider = new WsProvider('ws://127.0.0.1:8844');
    const api = await ApiPromise.create({provider: wsProvider});

    // simple query function
    let total_nft = await api.query.nftPallet.NextNFTId();
    console.log("total nft ", total_nft);

    // mint nft
    const keyring = new Keyring({type: 'sr25519'});
    const alice = keyring.addFromUri('//Alice');
    const transfer = api.tx.nftPallet.mintNft(hash);
    // Sign and send the transaction using our account
    const out = await transfer.signAndSend(alice);

    console.log('Transfer sent with hash', out.toHex());
}

main().catch(console.error).finally(() => process.exit());