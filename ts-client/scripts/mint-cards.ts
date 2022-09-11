import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { connect, sendTransactionAsync } from "../src/utils";

import "../src/interfaces/augment-api";

(async () => {
  let { api, alice } = await connect();

  const credits = await api.query.credits.credits.entries();
  for (const entry of credits) {
    if (entry[1].isSome) {
      const credit = entry[1].unwrap();
      console.log(`hahahahahahahahahaah ${credit.name.toUtf8()} ${credit}`);
      const id = entry[0].args[0];
      if (!credit.cardsMinted.toHuman()) {
        const tx = api.tx.credits.mintCards(id);
        await sendTransactionAsync(api, alice, tx, `mint cards for ${credit.name.toUtf8()}`);
      } else {
        console.log(`skipping ${credit.name.toUtf8()} because they already have cards minted`);
      }
    }
  }
})()

