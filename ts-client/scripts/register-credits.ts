import { ApiPromise, HttpProvider, Keyring, WsProvider } from "@polkadot/api";
import { AddressOrPair, ApiTypes, SubmittableExtrinsic } from "@polkadot/api/types";
import { Enum, u64 } from "@polkadot/types-codec";
import { ISubmittableResult } from "@polkadot/types/types";
import axios from "axios";
import { USER, TEST_USERS } from "./credits";
import { connect, sendTransactionAsync } from "../src/utils";
import "../src/interfaces/augment-api";
import { config } from "../src/config";

async function registerAccount(api: ApiPromise, signer: AddressOrPair, user: USER) {
	// fetch existing credits (registered and approved) to avoid registering duplicates
	await axios({
		method: "POST",
		url: `${config.CRS_BASE_URL}direct/user-reg`,
		headers: {
			Accept: "*/*",
			Authorization: `Bearer ${config.CRS_API_KEY}`,
			"Content-Type": "application/json",
		},
		data: {
			email: user.email,
			mobile: user.mobile,
			fname: user.fname,
			lname: user.lname,
			smsMsg: true,
			emailMsg: true,
			pushMsg: true,
		},
	})
		.then((resp) => {
      console.log(resp.data);
      console.log(`User ${user.fname+" "+user.lname} is now registered.`)
		})
		.catch((error) => {
			console.log(error.message);
			console.log(error.response.data.messages[0]);
		});
}

(async () => {
	let { api, alice, endpoint } = await connect();
	console.log(`Connected to node ${endpoint}`);
	let test_users = JSON.stringify(TEST_USERS[0], null, 2); // spacing level = 2
	console.log(`Begin registering users \n ${test_users}`);
	await registerAccount(api, alice, TEST_USERS[0]);
  process.exit()
})();
