import { ApiPromise, HttpProvider, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
import { Enum, u64 } from "@polkadot/types-codec";
import { MetaCreditPrimitivesCreditApplication } from "@polkadot/types/lookup";
import { ISubmittableResult } from "@polkadot/types/types";
import { Credit, TEST_CREDITS } from "./credits";
import { connect, sendTransactionAsync } from "../src/utils";

import "../src/interfaces/augment-api";

async function registerCredits(api: ApiPromise, signer: AddressOrPair, credits: Credit[]) {
  // fetch existing credits (registered and approved) to avoid registering duplicates
  const pending = await api.query.credits.applications.entries();
  const approved = await api.query.credits.credits.entries();

  const existingCredits: Set<string> = new Set();
  const approvedCredits: Set<string> = new Set();
  const creditIds: Map<string, u64> = new Map();

  for (const credit of pending) {
    if (credit[1].isSome) {
      // get the registered credit's id and name
      const id = credit[0].args[0];
      const name = credit[1].unwrap().name.toUtf8();
      existingCredits.add(name);
      creditIds.set(name, id);
    }
  }

  for (const credit of approved) {
    if (credit[1].isSome) {
      // get the approved credit's id and name
      const id = credit[0].args[0];
      const name = credit[1].unwrap().name.toUtf8();
      existingCredits.add(name);
      approvedCredits.add(name);
      creditIds.set(name, id);
    }
  }

  for (const credit of credits) {
    // skip credits that already exist as either approved or registered
    if (existingCredits.has(credit.name)) {
      console.log(`Skipping registration for ${credit.name}`)
      continue;
    }

    console.log(`Registering credit ${credit.name} ${credit.kind}`);
    const registerResult = await sendTransactionAsync(api, signer, api.tx.credits.submitCreditApplication({
      name: credit.name,
      applicantAccount: null,
      creditKind: credit.kind,
      photo: null
    }), `register ${credit.name}`);

    let id: u64 | undefined;
    for (const record of registerResult.events) {
      const event = record.event;
      if (api.events.credits.CreditApplicationSubmitted.is(event)) {
        id = event.data[0];
      }
    }

    if (!id) {
      throw "application submission failed";
    }
    creditIds.set(credit.name, id);
  }

  for (const credit of credits) {
    // skip credits that are already approved
    if (approvedCredits.has(credit.name)) {
      console.log(`Skipping approval for ${credit.name}`)
      continue;
    }

    console.log(`Approving credit ${credit.name}`);

    const id = creditIds.get(credit.name);
    if (!id) {
      throw `no id for credit ${credit.name}`
    }

    let approvalDone = false;
    const approvalResult = await sendTransactionAsync(api, signer, api.tx.sudo.sudo(api.tx.credits.approveApplication(id)), `approve ${credit.name}`);
    for (const record of approvalResult.events) {
      const event = record.event;
      if (api.events.credits.CreditApplicationApproved.is(event)) {
        approvalDone = true;
      }
    }

    if (!approvalDone) {
      throw "approval failed";
    }
  }
}

(async () => {
  let { api, alice } = await connect();

  await registerCredits(api, alice, TEST_CREDITS);
})()

