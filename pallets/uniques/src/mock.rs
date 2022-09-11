// This file is part of Substrate.

// Copyright (C) 2019-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test environment for Assets pallet.

use super::*;
use crate as pallet_uniques;

use frame_support::{
  construct_runtime,
  traits::{ConstU32, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
  testing::Header,
  traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
  pub enum Test where
    Block = Block,
    NodeBlock = Block,
    UncheckedExtrinsic = UncheckedExtrinsic,
  {
    System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
    Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
    Uniques: pallet_uniques::{Pallet, Call, Storage, Event<T>},
  }
);

impl frame_system::Config for Test {
  type AccountData = pallet_balances::AccountData<u64>;
  type AccountId = u64;
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockHashCount = ConstU64<250>;
  type BlockLength = ();
  type BlockNumber = u64;
  type BlockWeights = ();
  type Call = Call;
  type DbWeight = ();
  type Event = Event;
  type Hash = H256;
  type Hashing = BlakeTwo256;
  type Header = Header;
  type Index = u64;
  type Lookup = IdentityLookup<Self::AccountId>;
  type MaxConsumers = ConstU32<16>;
  type OnKilledAccount = ();
  type OnNewAccount = ();
  type OnSetCode = ();
  type Origin = Origin;
  type PalletInfo = PalletInfo;
  type SS58Prefix = ();
  type SystemWeightInfo = ();
  type Version = ();
}

impl pallet_balances::Config for Test {
  type AccountStore = System;
  type Balance = u64;
  type DustRemoval = ();
  type Event = Event;
  type ExistentialDeposit = ConstU64<1>;
  type MaxLocks = ();
  type MaxReserves = ConstU32<50>;
  type ReserveIdentifier = [u8; 8];
  type WeightInfo = ();
}

impl Config for Test {
  type AttributeDepositBase = ConstU64<1>;
  type ClassDeposit = ConstU64<2>;
  type ClassId = u32;
  type Currency = Balances;
  type DepositPerByte = ConstU64<1>;
  type Event = Event;
  type ForceOrigin = frame_system::EnsureRoot<u64>;
  type InstanceDeposit = ConstU64<1>;
  type InstanceId = u32;
  type KeyLimit = ConstU32<50>;
  type MetadataDepositBase = ConstU64<1>;
  type StringLimit = ConstU32<50>;
  type ValueLimit = ConstU32<50>;
  type WeightInfo = ();
}

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
  let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

  let mut ext = sp_io::TestExternalities::new(t);
  ext.execute_with(|| System::set_block_number(1));
  ext
}
