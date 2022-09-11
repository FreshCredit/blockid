import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { connect, sendTransactionAsync } from "../src/utils";
import { Enum, Option, u32 } from "@polkadot/types-codec"
import type { AccountId32, H256 } from '@polkadot/types/interfaces/runtime';

import { MetaCreditPrimitivesCard } from "@polkadot/types/lookup";

function getRandomInt(min: number, max: number): number {
  min = Math.ceil(min);
  max = Math.floor(max);
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

function printCard(credit: string, hex: H256, card: MetaCreditPrimitivesCard) {
  console.log(`Card from onchain for ${credit}: (${hex.toHex()}): tier=${card.tier.toString()} value=${card.value.toNumber() / 1_000_000_000_000} ATHL`);
}

function printGetCard( credit: string, id: number, cards: Map<number, [H256, MetaCreditPrimitivesCard]>) {
  let card = cards.get(id)!;
  printCard(credit, card[0], card[1]);
}

(async () => {
  let { api, alice } = await connect();


  api.registerTypes({
    CreditCardClass: {
      _enum: ['Gold', 'Platinum', 'Diamond']
    }
  })

  const credits = await api.query.credits.credits.entries();
  const cards = await api.query.credits.cards.entries();

  for (const entry of credits) {
    if (entry[1].isSome) {
      const credit = entry[1].unwrap();
      const creditId = entry[0].args[0];
      const name = credit.name.toUtf8();

      console.log(`hahahahahahahah for ${credit.creditKind}`);
      const kind = credit.creditKind.toUtf8();
      console.log(`Registered credit ${name}: kind=${kind}`)
      console.log(`Cards minted for ${name}: ${credit.cardsMinted}`)

      let goldCards = 0;
      let platinumCards = 0;
      let diamondCards = 0;

      const creditCards: Map<number, [H256, MetaCreditPrimitivesCard]> = new Map();
      for (const entry of cards) {
        const key = entry[0].args[0];
        const opt = entry[1] as Option<MetaCreditPrimitivesCard>;
        if (opt.isSome) {
          const card = opt.unwrap();

          if (card.id.creditId.toHuman() != creditId.toHuman()) {
            continue
          }

          if (card.tier.isGold) {
            goldCards += 1;
          } else if (card.tier.isPlatinum) {
            platinumCards += 1;
          } else if (card.tier.isDiamond) {
            diamondCards += 1;
          }

          creditCards.set(card.id.instanceId.toNumber(), [key, card]);
        }
      }

      console.log(`Gold: ${goldCards}; Platinum: ${platinumCards}; Diamond: ${diamondCards};`);
      printGetCard(name, 1, creditCards);
      printGetCard(name, 20, creditCards);
      printGetCard(name, 110, creditCards);

      console.log();
    }
  }
})()

