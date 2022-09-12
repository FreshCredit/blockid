import { ApiPromise, HttpProvider, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
import { Enum, u64 } from "@polkadot/types-codec";
import { MetaCreditPrimitivesCreditApplication } from "@polkadot/types/lookup";
import { ISubmittableResult } from "@polkadot/types/types";
import axios from 'axios';
import { Credit, TEST_CREDITS } from "./credits";
import { connect, sendTransactionAsync } from "../src/utils";
import "../src/interfaces/augment-api";
import { config } from '../src/config';

async function registerCredits(api: ApiPromise, signer: AddressOrPair, credits: Credit[]) {
  // fetch existing credits (registered and approved) to avoid registering duplicates
  try {
    for (const user of TEST_CREDITS) {
      const resp = await axios({
        method: 'post',
        url: `${config.BASE_URL}direct/user-reg`,
        headers: {
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${config.API_KEY}`
        },
        data: {
          email: user.email,
          mobile: user.mobile,
          fname: user.fname,
          lname: user.lname,
          smsMsg: true,
          emailMsg: true,
          pushMsg: true
        }
        
      });
      console.log('[Register User] = ', resp);
    }
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

(async () => {
  let { api, alice } = await connect();

  await registerCredits(api, alice, TEST_CREDITS);
})()

