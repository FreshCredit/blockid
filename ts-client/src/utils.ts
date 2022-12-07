import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, SubmittableExtrinsic } from "@polkadot/api/types";
import { ISubmittableResult } from "@polkadot/types/types";

import "./interfaces/augment-api";
import "../src/interfaces/types-lookup";

const WAIT_FOR_FINALIZATION = false;
const NODE_ENDPOINT = 'ws://127.0.0.1:9944'
// const NODE_ENDPOINT = 'ws://95.217.34.239:9944'

export async function connect(): Promise<{api: ApiPromise, keyring: Keyring, alice: AddressOrPair}> {
  const wsProvider = new WsProvider(NODE_ENDPOINT);
  const api = await ApiPromise.create({ provider: wsProvider });

  const keyring = new Keyring({ type: 'sr25519' });
  const alice = keyring.addFromUri('//Alice', { name: 'Alice' });
  // const alice = keyring.addFromMnemonic('better stamp learn book patrol toast enough task survey hire derive garment', { name: 'TEST' });

  return {api, keyring, alice}
}

export async function sendTransactionAsync(api: ApiPromise, account: AddressOrPair, extrinsic: SubmittableExtrinsic<"promise">, tag?: string): Promise<ISubmittableResult> {
  let resolveFn: any;
  let rejectFn: any;
  const promise = new Promise((resolve, reject) => {resolveFn = resolve; rejectFn = reject});
  const transactionTag = tag ? tag : "unknown";

  const unsub = await extrinsic.signAndSend(account, (result) => {
    console.log(`[${transactionTag}] Current status is ${result.status}`);

    let errorFound = false;
    if (result.isInBlock || result.isFinalized) {
      for (const record of result.events) {
        const event = record.event;
        if (api.events.system.ExtrinsicFailed.is(event)) {
          const error = event.data[0];
          if (error.isModule) {
            const decoded = api.registry.findMetaError(error.asModule);
            const { docs, method, section } = decoded;

            console.log(`[${transactionTag}] ${section}.${method}: ${docs.join(' ')}`);
          } else {
            console.log(`[${transactionTag}] ${error.toString()}`);
          }

          errorFound = true;
        }
      }
    }

    if (errorFound) {
      unsub();
      rejectFn("error occured in extrinsic");
    }

    if (result.isError) {
      console.log(`[${transactionTag}] Transaction failed: ${result}`);
      rejectFn("transaction failed");
    } else if (result.status.isInBlock) {
      console.log(`[${transactionTag}] Transaction included at blockHash ${result.status.asInBlock}`);
      if (!WAIT_FOR_FINALIZATION) {
        unsub();
        resolveFn(result)
      }
    } else if (result.status.isFinalized) {
      console.log(`[${transactionTag}] Transaction finalized at blockHash ${result.status.asFinalized}`);
      if (WAIT_FOR_FINALIZATION) {
        unsub();
        resolveFn(result)
      }
    }
  });

  const result = (await promise) as ISubmittableResult;
  return result;
}
