import { ApiPromise, HttpProvider, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
import { Enum, u64 } from "@polkadot/types-codec";
import { MetaCreditPrimitivesCreditApplication } from "@polkadot/types/lookup";
import { ISubmittableResult } from "@polkadot/types/types";
import axios from 'axios';
import { USER, TEST_USERS } from "./credits";
import { connect, sendTransactionAsync } from "../src/utils";
import "../src/interfaces/augment-api";
import { config } from '../src/config';

async function registerAccount(api: ApiPromise, signer: AddressOrPair, user: USER) {
  // fetch existing credits (registered and approved) to avoid registering duplicates
  try {
    const resp = await axios({
      method: 'post',
      url: `${config.CRS_BASE_URL}direct/user-reg`,
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${config.CRS_API_KEY}`
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
  } catch (error) {
    console.log('[Error] = ', error);
  }
}

(async () => {
  let { api, alice } = await connect();
  console.log(alice);
  await registerAccount(api, alice, TEST_USERS[0]);
})()