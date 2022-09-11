import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, SubmittableExtrinsic } from "@polkadot/api/types";
import { ISubmittableResult } from "@polkadot/types/types";

/**
 * High-level wrapper for a connection with a MetaCredit node.
 */
export class MAConnection {
  /**
   * Handle to the Polkadot.js {@link ApiPromise} object used by the wrapper.
   */
  public api: ApiPromise;
  /**
   * Handle to the Polkadot.js {@link Keyring} object used by the wrapper.
   */
  public keyring: Keyring;

  /**
   * Whether the client should wait for transactions to be finalized when they are sent.
   */
  public waitForFinalization: boolean = false;

  private constructor(api: ApiPromise, keyring: Keyring) {
    this.api = api;
    this.keyring = keyring;
  }

  /**
   * Connect to a node at the specified WebSocket endpoint (ws or wss).
   */
  static async connect(endpoint: string): Promise<MAConnection> {
    const wsProvider = new WsProvider(endpoint);
    const api = await ApiPromise.create({ provider: wsProvider });
    const keyring = new Keyring({type: "sr25519"});

    return new MAConnection(api, keyring)
  }

  async sendTransactionAsync(account: AddressOrPair, extrinsic: SubmittableExtrinsic<"promise">, tag?: string): Promise<ISubmittableResult> {
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
          if (this.api.events.system.ExtrinsicFailed.is(event)) {
            const error = event.data[0];
            if (error.isModule) {
              const decoded = this.api.registry.findMetaError(error.asModule);
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
        if (!this.waitForFinalization) {
          unsub();
          resolveFn(result)
        }
      } else if (result.status.isFinalized) {
        console.log(`[${transactionTag}] Transaction finalized at blockHash ${result.status.asFinalized}`);
        if (this.waitForFinalization) {
          unsub();
          resolveFn(result)
        }
      }
    });

    const result = (await promise) as ISubmittableResult;
    return result;
  }
}
