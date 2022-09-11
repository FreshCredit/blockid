#![cfg_attr(not(feature = "std"), no_std)]
#![recursion_limit = "256"]

extern crate alloc;

pub mod opaque;

#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use alloc::{boxed::Box, vec::Vec};
use frame_support::{
  construct_runtime, parameter_types,
  traits::{ConstU128, ConstU16, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem},
  weights::{
    constants::{RocksDbWeight, WEIGHT_PER_SECOND},
    IdentityFee,
  },
};
use frame_system::EnsureRoot;
use fresh_credit_primitives::{Balance, InstanceId};
use pallet_grandpa::{
  fg_primitives, AuthorityId as GrandpaId, AuthorityList as GrandpaAuthorityList,
};
use pallet_transaction_payment::CurrencyAdapter;
use sp_api::impl_runtime_apis;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
use sp_runtime::{
  create_runtime_str, generic,
  traits::{AccountIdLookup, BlakeTwo256, Block as BlockT, IdentifyAccount, NumberFor, Verify},
  transaction_validity::{TransactionSource, TransactionValidity},
  ApplyExtrinsicResult, MultiSignature, MultiSigner, Perbill,
};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
pub type Address = sp_runtime::MultiAddress<AccountId, ()>;
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
pub type BlockNumber = u32;
pub type Executive = frame_executive::Executive<
  Runtime,
  Block,
  frame_system::ChainContext<Runtime>,
  Runtime,
  AllPalletsWithSystem,
>;
pub type Hash = sp_core::H256;
pub type Index = u32;
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
pub type SignedExtra = (
  frame_system::CheckSpecVersion<Runtime>,
  frame_system::CheckTxVersion<Runtime>,
  frame_system::CheckGenesis<Runtime>,
  frame_system::CheckEra<Runtime>,
  frame_system::CheckNonce<Runtime>,
  frame_system::CheckWeight<Runtime>,
  pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);
pub type Signature = MultiSignature;
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

pub const DAYS: BlockNumber = HOURS * 24;
pub const HOURS: BlockNumber = MINUTES * 60;
pub const MILLISECS_PER_BLOCK: u64 = 6000;
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
  apis: RUNTIME_API_VERSIONS,
  authoring_version: 1,
  impl_name: create_runtime_str!("fresh-credit"),
  impl_version: 1,
  spec_name: create_runtime_str!("fresh-credit"),
  spec_version: 100,
  state_version: 1,
  transaction_version: 1,
};

construct_runtime!(
  pub enum Runtime where
    Block = Block,
    NodeBlock = opaque::Block,
    UncheckedExtrinsic = UncheckedExtrinsic
  {
    Credits: pallet_credits,
    Aura: pallet_aura,
    Balances: pallet_balances,
    Grandpa: pallet_grandpa,
    RandomnessCollectiveFlip: pallet_randomness_collective_flip,
    Sudo: pallet_sudo,
    System: frame_system,
    Timestamp: pallet_timestamp,
    TransactionPayment: pallet_transaction_payment,
    Uniques: pallet_uniques::{Pallet, Event<T>, Storage},
  }
);

impl<LC> frame_system::offchain::SendTransactionTypes<LC> for Runtime
where
  Call: From<LC>,
{
  type Extrinsic = UncheckedExtrinsic;
  type OverarchingCall = Call;
}

impl frame_system::offchain::SigningTypes for Runtime {
  type Public = <Signature as Verify>::Signer;
  type Signature = Signature;
}

parameter_types! {
  pub SystemAccountId: AccountId = AccountId::new([0u8; 32]);
  pub InitialCardValues: fresh_credit_primitives::InitialCardValues<Balance> = fresh_credit_primitives::InitialCardValues {
    gold: 5_000_000_000_000,
    platinum: 20_000_000_000_000,
    diamond: 100_000_000_000_000,
  };
}

impl pallet_credits::Config for Runtime {
  type Card = Uniques;
  type Currency = Balances;
  type Event = Event;
  type InitialCardValues = InitialCardValues;
  type OffchainAuthority = OffchainAppCrypto;
  type OffchainUnsignedGracePeriod = ConstU32<5>;
  type OffchainUnsignedInterval = ConstU32<128>;
  type SystemAccountId = SystemAccountId;
}

impl pallet_balances::Config for Runtime {
  type AccountStore = System;
  type Balance = Balance;
  type DustRemoval = ();
  type Event = Event;
  type ExistentialDeposit = ConstU128<500>;
  type MaxLocks = ConstU32<50>;
  type MaxReserves = ();
  type ReserveIdentifier = [u8; 8];
  type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);
parameter_types! {
  pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
  pub BlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::with_sensible_defaults(2 * WEIGHT_PER_SECOND, NORMAL_DISPATCH_RATIO);
  pub const Version: RuntimeVersion = VERSION;
}
impl frame_system::Config for Runtime {
  type AccountData = pallet_balances::AccountData<Balance>;
  type AccountId = AccountId;
  type BaseCallFilter = frame_support::traits::Everything;
  type BlockHashCount = ConstU32<2400>;
  type BlockLength = BlockLength;
  type BlockNumber = BlockNumber;
  type BlockWeights = BlockWeights;
  type Call = Call;
  type DbWeight = RocksDbWeight;
  type Event = Event;
  type Hash = Hash;
  type Hashing = BlakeTwo256;
  type Header = generic::Header<BlockNumber, BlakeTwo256>;
  type Index = Index;
  type Lookup = AccountIdLookup<AccountId, ()>;
  type MaxConsumers = frame_support::traits::ConstU32<16>;
  type OnKilledAccount = ();
  type OnNewAccount = ();
  type OnSetCode = ();
  type Origin = Origin;
  type PalletInfo = PalletInfo;
  type SS58Prefix = ConstU16<42>;
  type SystemWeightInfo = ();
  type Version = Version;
}

pub type MaxAuthorities = ConstU32<32>;

impl pallet_aura::Config for Runtime {
  type AuthorityId = AuraId;
  type DisabledValidators = ();
  type MaxAuthorities = MaxAuthorities;
}

impl pallet_grandpa::Config for Runtime {
  type Call = Call;
  type Event = Event;
  type HandleEquivocation = ();
  type KeyOwnerIdentification =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::IdentificationTuple;
  type KeyOwnerProof =
    <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
  type KeyOwnerProofSystem = ();
  type MaxAuthorities = MaxAuthorities;
  type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Runtime {}

impl pallet_sudo::Config for Runtime {
  type Call = Call;
  type Event = Event;
}

impl pallet_timestamp::Config for Runtime {
  type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
  type Moment = u64;
  type OnTimestampSet = Aura;
  type WeightInfo = ();
}

impl pallet_transaction_payment::Config for Runtime {
  type FeeMultiplierUpdate = ();
  type OnChargeTransaction = CurrencyAdapter<Balances, ()>;
  type OperationalFeeMultiplier = ConstU8<5>;
  type TransactionByteFee = ConstU128<1>;
  type WeightToFee = IdentityFee<Balance>;
}

impl pallet_uniques::Config for Runtime {
  type AttributeDepositBase = ConstU128<1>;
  type ClassDeposit = ConstU128<0>;
  type ClassId = u64;
  type Currency = Balances;
  type DepositPerByte = ConstU128<0>;
  type Event = Event;
  type ForceOrigin = EnsureRoot<AccountId>;
  type InstanceDeposit = ConstU128<0>;
  type InstanceId = InstanceId;
  type KeyLimit = ConstU32<50>;
  type MetadataDepositBase = ConstU128<0>;
  type StringLimit = ConstU32<50>;
  type ValueLimit = ConstU32<50>;
  type WeightInfo = ();
}

impl_runtime_apis! {
  impl sp_api::Core<Block> for Runtime {
    fn execute_block(block: Block) {
      Executive::execute_block(block);
    }

    fn initialize_block(header: &<Block as BlockT>::Header) {
      Executive::initialize_block(header)
    }

    fn version() -> RuntimeVersion {
      VERSION
    }
  }

  impl sp_api::Metadata<Block> for Runtime {
    fn metadata() -> OpaqueMetadata {
      OpaqueMetadata::new(Runtime::metadata().into())
    }
  }

  impl sp_block_builder::BlockBuilder<Block> for Runtime {
    fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
      Executive::apply_extrinsic(extrinsic)
    }

    fn check_inherents(
      block: Block,
      data: sp_inherents::InherentData,
    ) -> sp_inherents::CheckInherentsResult {
      data.check_extrinsics(&block)
    }

    fn finalize_block() -> <Block as BlockT>::Header {
      Executive::finalize_block()
    }

    fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
      data.create_extrinsics()
    }
  }

  impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
    fn validate_transaction(
      source: TransactionSource,
      tx: <Block as BlockT>::Extrinsic,
      block_hash: <Block as BlockT>::Hash,
    ) -> TransactionValidity {
      Executive::validate_transaction(source, tx, block_hash)
    }
  }

  impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
    fn offchain_worker(header: &<Block as BlockT>::Header) {
      Executive::offchain_worker(header)
    }
  }

  impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
    fn authorities() -> Vec<AuraId> {
      Aura::authorities().into_inner()
    }

    fn slot_duration() -> sp_consensus_aura::SlotDuration {
      sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
    }
  }

  impl sp_session::SessionKeys<Block> for Runtime {
    fn decode_session_keys(
      encoded: Vec<u8>,
    ) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
      opaque::SessionKeys::decode_into_raw_public_keys(&encoded)
    }

    fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
      opaque::SessionKeys::generate(seed)
    }
  }

  impl fg_primitives::GrandpaApi<Block> for Runtime {
    fn current_set_id() -> fg_primitives::SetId {
      Grandpa::current_set_id()
    }

    fn generate_key_ownership_proof(
      _set_id: fg_primitives::SetId,
      _authority_id: GrandpaId,
    ) -> Option<fg_primitives::OpaqueKeyOwnershipProof> {
      None
    }

    fn grandpa_authorities() -> GrandpaAuthorityList {
      Grandpa::grandpa_authorities()
    }

    fn submit_report_equivocation_unsigned_extrinsic(
      _equivocation_proof: fg_primitives::EquivocationProof<
        <Block as BlockT>::Hash,
        NumberFor<Block>,
      >,
      _key_owner_proof: fg_primitives::OpaqueKeyOwnershipProof,
    ) -> Option<()> {
      None
    }
  }

  impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
    fn account_nonce(account: AccountId) -> Index {
      System::account_nonce(account)
    }
  }

  impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
    fn query_fee_details(
      uxt: <Block as BlockT>::Extrinsic,
      len: u32,
    ) -> pallet_transaction_payment::FeeDetails<Balance> {
      TransactionPayment::query_fee_details(uxt, len)
    }

    fn query_info(
      uxt: <Block as BlockT>::Extrinsic,
      len: u32,
    ) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
      TransactionPayment::query_info(uxt, len)
    }
  }

  #[cfg(feature = "runtime-benchmarks")]
  impl frame_benchmarking::Benchmark<Block> for Runtime {
    fn benchmark_metadata(extra: bool) -> (
      Vec<frame_benchmarking::BenchmarkList>,
      Vec<frame_support::traits::StorageInfo>,
    ) {
      use frame_benchmarking::{list_benchmark, baseline, Benchmarking, BenchmarkList};
      use frame_support::traits::StorageInfoTrait;
      use frame_system_benchmarking::Pallet as SystemBench;
      use baseline::Pallet as BaselineBench;

      let mut list = Vec::<BenchmarkList>::new();

      list_benchmark!(list, extra, frame_benchmarking, BaselineBench::<Runtime>);
      list_benchmark!(list, extra, frame_system, SystemBench::<Runtime>);
      list_benchmark!(list, extra, pallet_balances, Balances);
      list_benchmark!(list, extra, pallet_timestamp, Timestamp);

      let storage_info = AllPalletsWithSystem::storage_info();

      (list, storage_info)
    }

    fn dispatch_benchmark(
      config: frame_benchmarking::BenchmarkConfig
    ) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
      use frame_benchmarking::{baseline, Benchmarking, BenchmarkBatch, add_benchmark, TrackedStorageKey};

      use frame_system_benchmarking::Pallet as SystemBench;
      use baseline::Pallet as BaselineBench;

      impl frame_system_benchmarking::Config for Runtime {}
      impl baseline::Config for Runtime {}

      let whitelist: Vec<TrackedStorageKey> = alloc::vec![
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
        hex_literal::hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
        hex_literal::hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
      ];

      let mut batches = Vec::<BenchmarkBatch>::new();
      let params = (&config, &whitelist);

      add_benchmark!(params, batches, frame_benchmarking, BaselineBench::<Runtime>);
      add_benchmark!(params, batches, frame_system, SystemBench::<Runtime>);
      add_benchmark!(params, batches, pallet_balances, Balances);
      add_benchmark!(params, batches, pallet_timestamp, Timestamp);

      Ok(batches)
    }
  }
}

pub struct OffchainAppCrypto;

impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for OffchainAppCrypto {
  type GenericPublic = sp_runtime::app_crypto::sr25519::Public;
  type GenericSignature = sp_runtime::app_crypto::sr25519::Signature;
  type RuntimeAppPublic = pallet_credits::Public;
}

#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
  NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}
