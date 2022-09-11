#![cfg(test)]

use crate::{
  mock::{
    alice, bob, test_ext, Credits, Balances, Event, Extrinsic, Origin, Runtime, System, TestAuth,
  },
  Call, Error, OffchainPayload, ORACLE_URL,
};
use alloc::sync::Arc;
use frame_support::assert_noop;
use frame_system::offchain::{SignedPayload, SigningTypes};
use fresh_credit_primitives::{Credit, CreditApplication, CreditCardClass};
use parity_scale_codec::Decode;
use sp_core::offchain::{testing, OffchainWorkerExt, TransactionPoolExt};
use sp_io::TestExternalities;
use sp_keystore::{testing::KeyStore, KeystoreExt, SyncCryptoStore};
use sp_runtime::RuntimeAppPublic;

#[test]
fn mint_cards_generates_class_and_cards() {
  test_ext().execute_with(|| {
    System::set_block_number(1);

    dbg!(Balances::free_balance(alice()));
    dbg!(Balances::usable_balance(alice()));

    Credits::submit_credit_application(Origin::signed(alice()), credit_application(bob()))
      .unwrap();

    let credit_id = System::events()
      .iter()
      .filter_map(|record| {
        let event = &record.event;
        if let Event::Credits(crate::Event::CreditApplicationSubmitted { id, .. }) = event {
          Some(*id)
        } else {
          None
        }
      })
      .next()
      .unwrap();

    Credits::approve_application(Origin::root(), credit_id).unwrap();
    Credits::mint_cards(Origin::signed(alice()), credit_id).unwrap();

    dbg!(System::events());

    // Credits::mint_cards(Origin::signed(alice()), credit()).unwrap();
    // assert_eq!(Credits::class_attributes(&0).unwrap(), credit());
    // assert_eq!(
    //   Credits::attributes(&0, &0).unwrap(),
    //   CreditCardAttributes {
    //     price: None,
    //     total_shares: 0,
    //     ty: CreditCardClass::Platinum,
    //     views: 0,
    //     votes: 0,
    //   }
    // );
    // assert_eq!(
    //   Credits::attributes(&0, &100).unwrap(),
    //   CreditCardAttributes {
    //     price: None,
    //     total_shares: 0,
    //     ty: CreditCardClass::Gold,
    //     views: 0,
    //     votes: 0,
    //   }
    // );
    // assert_eq!(
    //   Credits::attributes(&0, &140).unwrap(),
    //   CreditCardAttributes {
    //     price: None,
    //     total_shares: 0,
    //     ty: CreditCardClass::Platinum,
    //     views: 0,
    //     votes: 0,
    //   }
    // );
  });
}

// #[test]
// fn offchain_worker_submits_unsigned_transaction_on_chain() {
//   test_ext().execute_with(|| {
//     const SEED: Option<&str> =
//       Some("news slush supreme milk chapter credit soap sausage put clutch what kitten/foo");

//     let (offchain, offchain_state) = testing::TestOffchainExt::new();

//     let (pool, pool_state) = testing::TestTransactionPoolExt::new();

//     let keystore = KeyStore::new();

//     let public_key =
//       SyncCryptoStore::sr25519_generate_new(&keystore, crate::Public::ID, SEED).unwrap();

//     let mut t = TestExternalities::default();
//     t.register_extension(OffchainWorkerExt::new(offchain));
//     t.register_extension(TransactionPoolExt::new(pool));
//     t.register_extension(KeystoreExt(Arc::new(keystore)));

//     offchain_state.write().expect_request(testing::PendingRequest {
//       method: "GET".into(),
//       uri: ORACLE_URL.into(),
//       response: Some(br#"0,1,10,20"#.to_vec()),
//       sent: true,
//       ..Default::default()
//     });

//     let payload = OffchainPayload {
//       class_id: 0,
//       instance_id: 1,
//       views: 10,
//       votes: 20,
//       public: <Runtime as SigningTypes>::Public::from(public_key),
//     };

//     t.execute_with(|| {
//       // when
//       Credits::fetch_pair_prices_and_submit_tx().unwrap();

//       // then
//       let raw_tx = pool_state.write().transactions.pop().unwrap();
//       let tx = Extrinsic::decode(&mut &*raw_tx).unwrap();
//       assert_eq!(tx.signature, None);
//       if let Call::submit_credit_info { class_id, instance_id, views, votes, signature } = tx.call
//       {
//         assert_eq!(class_id, payload.class_id);
//         assert_eq!(instance_id, payload.instance_id);
//         assert_eq!(views, payload.views);
//         assert_eq!(votes, payload.votes);
//         let signature_valid =
//           <OffchainPayload<<Runtime as SigningTypes>::Public> as SignedPayload<Runtime>>::verify::<
//             TestAuth,
//           >(&payload, signature);
//         assert!(signature_valid);
//       }
//     });
//   })
// }

// #[test]
// fn set_card_price_does_not_modifies_non_owned_cards() {
//   test_ext().execute_with(|| {
//     Credits::mint_cards(Origin::signed(alice()), credit()).unwrap();
//     assert_noop!(
//       Credits::set_card_price(Origin::signed(bob()), 0, 0, Some(10)),
//       Error::<Runtime>::MustBeCardOwner
//     );
//   });
// }

// #[test]
// fn set_card_price_modifies_stored_prices() {
//   test_ext().execute_with(|| {
//     Credits::mint_cards(Origin::signed(alice()), credit()).unwrap();
//     assert_eq!(Credits::attributes(&0, &0).unwrap().price, None);
//     Credits::set_card_price(Origin::signed(alice()), 0, 0, Some(10)).unwrap();
//     assert_eq!(Credits::attributes(&0, &0).unwrap().price, Some(10));
//     Credits::set_card_price(Origin::signed(alice()), 0, 0, None).unwrap();
//     assert_eq!(Credits::attributes(&0, &0).unwrap().price, None);
//   });
// }

// #[test]
// fn transfer_card_does_not_modifies_cards_that_are_not_for_sale() {
//   test_ext().execute_with(|| {
//     Credits::mint_cards(Origin::signed(alice()), credit()).unwrap();
//     assert_noop!(
//       Credits::transfer_card(Origin::signed(bob()), 0, 0),
//       Error::<Runtime>::CardIsNotForSale
//     );
//   });
// }

// #[test]
// fn transfer_card_modifies_balances_and_sends_card_to_caller() {
//   test_ext().execute_with(|| {
//     Credits::mint_cards(Origin::signed(alice()), credit()).unwrap();
//     Credits::set_card_price(Origin::signed(alice()), 0, 0, Some(10)).unwrap();
//     let previous_alice_balance = Balances::free_balance(alice());
//     assert_eq!(Credits::owner(&0, &0).unwrap(), alice());
//     Credits::transfer_card(Origin::signed(bob()), 0, 0).unwrap();
//     assert_eq!(990, Balances::free_balance(bob()));
//     assert_eq!(previous_alice_balance + 10, Balances::free_balance(alice()));
//     assert_eq!(Credits::owner(&0, &0).unwrap(), bob());
//   });
// }

// fn credit() -> Credit {
//   Credit {
//     name: "John J. Junior".into(),
//     photo: "iVBORw0KGgoAAAANSUhEUgAAABkAAAAZCAYAAADE6YVjAAAAGXRFWHRTb2Z0d2FyZQBBZG9iZSBJbWFnZVJl\
//       YWR5ccllPAAAAyJpVFh0WE1MOmNvbS5hZG9iZS54bXAAAAAAADw/eHBhY2tldCBiZWdpbj0i77u/IiBpZD0iVzVNME1w\
//       Q2VoaUh6cmVTek5UY3prYzlkIj8+IDx4OnhtcG1ldGEgeG1sbnM6eD0iYWRvYmU6bnM6bWV0YS8iIHg6eG1wdGs9IkFk\
//       b2JlIFhNUCBDb3JlIDUuMy1jMDExIDY2LjE0NTY2MSwgMjAxMi8wMi8wNi0xNDo1NjoyNyAgICAgICAgIj4gPHJkZjpS\
//       REYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4gPHJkZjpEZXNj\
//       cmlwdGlvbiByZGY6YWJvdXQ9IiIgeG1sbnM6eG1wPSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvIiB4bWxuczp4\
//       bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIgeG1sbnM6c3RSZWY9Imh0dHA6Ly9ucy5hZG9iZS5j\
//       b20veGFwLzEuMC9zVHlwZS9SZXNvdXJjZVJlZiMiIHhtcDpDcmVhdG9yVG9vbD0iQWRvYmUgUGhvdG9zaG9wIENTNiAo\
//       V2luZG93cykiIHhtcE1NOkluc3RhbmNlSUQ9InhtcC5paWQ6MEVBMTczNDg3QzA5MTFFNjk3ODM5NjQyRjE2RjA3QTki\
//       IHhtcE1NOkRvY3VtZW50SUQ9InhtcC5kaWQ6MEVBMTczNDk3QzA5MTFFNjk3ODM5NjQyRjE2RjA3QTkiPiA8eG1wTU06\
//       RGVyaXZlZEZyb20gc3RSZWY6aW5zdGFuY2VJRD0ieG1wLmlpZDowRUExNzM0NjdDMDkxMUU2OTc4Mzk2NDJGMTZGMDdB\
//       OSIgc3RSZWY6ZG9jdW1lbnRJRD0ieG1wLmRpZDowRUExNzM0NzdDMDkxMUU2OTc4Mzk2NDJGMTZGMDdBOSIvPiA8L3Jk\
//       ZjpEZXNjcmlwdGlvbj4gPC9yZGY6UkRGPiA8L3g6eG1wbWV0YT4gPD94cGFja2V0IGVuZD0iciI/PjjUmssAAAGASURB\
//       VHjatJaxTsMwEIbpIzDA6FaMMPYJkDKzVYU+QFeEGPIKfYU8AETkCYI6wANkZQwIKRNDB1hA0Jrf0rk6WXZ8BvWkb4kv\
//       99vn89kDrfVexBSYgVNwDA7AN+jAK3gEd+AlGMGIBFDgFvzouK3JV/lihQTOwLtOtw9wIRG5pJn91Tbgqk9kSk7GViAD\
//       rTD4HCyZ0NQnomi51sb0fUyCMQEbp2WpU67IjfNjwcYyoUDhjJVcZBjYBy40j4wXgaobWoe8Z6Y80CJBwFpunepIzt2A\
//       UgFjtXXshNXjVmMh+K+zzp/CMs0CqeuzrxSRpbOKfdCkiMTS1VBQ41uxMyQR2qbrXiiwYN3ACh1FDmsdK2Eu4J6Tlo31\
//       dYVtCY88h5ELZIJJ+IRMzBHfyJINrigNkt5VsRiub9nXICdsYyVd2NcVvA3ScE5t2rb5JuEeyZnAhmLt9NK63vX1O5Pe\
//       8XaPSuGq1uTrfUgMEp9EJ+CQvr+BJ/AAKvAcCiAR+bf9CjAAluzmdX4AEIIAAAAASUVORK5CYII="
//       .into(),
//   }
// }

fn credit_application<T>(account: T) -> CreditApplication<T> {
  CreditApplication {
    name: "John Doe".into(),
    applicant_account: account,
    creditKind: 0,
    photo: None,
  }
}
