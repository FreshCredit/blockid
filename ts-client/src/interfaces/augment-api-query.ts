// Auto-generated via `yarn polkadot-types-from-chain`, do not edit
/* eslint-disable */

import type { ApiTypes } from "@polkadot/api-base/types";
import type { Bytes, Null, Option, Vec, bool, u128, u32, u64 } from "@polkadot/types-codec";
import type { AnyNumber, ITuple } from "@polkadot/types-codec/types";
import type { AccountId32, H256 } from "@polkadot/types/interfaces/runtime";
import type {
	FrameSupportWeightsPerDispatchClassU64,
	FrameSystemAccountInfo,
	FrameSystemEventRecord,
	FrameSystemLastRuntimeUpgradeInfo,
	FrameSystemPhase,
	PalletBalancesAccountData,
	PalletBalancesBalanceLock,
	PalletBalancesReleases,
	PalletBalancesReserveData,
	PalletGrandpaStoredPendingChange,
	PalletGrandpaStoredState,
	PalletTransactionPaymentReleases,
	PalletUniquesClassDetails,
	PalletUniquesClassMetadata,
	PalletUniquesInstanceDetails,
	PalletUniquesInstanceMetadata,
	SpConsensusAuraSr25519AppSr25519Public,
	SpRuntimeDigest,
  MetaAthletePrimitivesAthleteApplication,
  MetaAthletePrimitivesCard,
  MetaAthletePrimitivesAthlete
} from "@polkadot/types/lookup";
import type { Observable } from "@polkadot/types/types";

declare module "@polkadot/api-base/types/storage" {
	export interface AugmentedQueries<ApiType extends ApiTypes> {
		athletes: {
			applications: AugmentedQuery<
				ApiType,
				(
					arg: u64 | AnyNumber | Uint8Array
				) => Observable<Option<MetaAthletePrimitivesAthleteApplication>>,
				[u64]
			> &
				QueryableStorageEntry<ApiType, [u64]>;
			athleteCounter: AugmentedQuery<ApiType, () => Observable<Option<u64>>, []> &
				QueryableStorageEntry<ApiType, []>;
			athletes: AugmentedQuery<
				ApiType,
				(
					arg: u64 | AnyNumber | Uint8Array
				) => Observable<Option<MetaAthletePrimitivesAthlete>>,
				[u64]
			> &
				QueryableStorageEntry<ApiType, [u64]>;
			cards: AugmentedQuery<
				ApiType,
				(arg: H256 | string | Uint8Array) => Observable<Option<MetaAthletePrimitivesCard>>,
				[H256]
			> &
				QueryableStorageEntry<ApiType, [H256]>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		aura: {
			/**
			 * The current authority set.
			 **/
			authorities: AugmentedQuery<
				ApiType,
				() => Observable<Vec<SpConsensusAuraSr25519AppSr25519Public>>,
				[]
			> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * The current slot of this block.
			 *
			 * This will be set in `on_initialize`.
			 **/
			currentSlot: AugmentedQuery<ApiType, () => Observable<u64>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		balances: {
			/**
			 * The balance of an account.
			 *
			 * NOTE: This is only used in the case that this pallet is used to store balances.
			 **/
			account: AugmentedQuery<
				ApiType,
				(arg: AccountId32 | string | Uint8Array) => Observable<PalletBalancesAccountData>,
				[AccountId32]
			> &
				QueryableStorageEntry<ApiType, [AccountId32]>;
			/**
			 * Any liquidity locks on some account balances.
			 * NOTE: Should only be accessed when setting, changing and freeing a lock.
			 **/
			locks: AugmentedQuery<
				ApiType,
				(
					arg: AccountId32 | string | Uint8Array
				) => Observable<Vec<PalletBalancesBalanceLock>>,
				[AccountId32]
			> &
				QueryableStorageEntry<ApiType, [AccountId32]>;
			/**
			 * Named reserves on some account balances.
			 **/
			reserves: AugmentedQuery<
				ApiType,
				(
					arg: AccountId32 | string | Uint8Array
				) => Observable<Vec<PalletBalancesReserveData>>,
				[AccountId32]
			> &
				QueryableStorageEntry<ApiType, [AccountId32]>;
			/**
			 * Storage version of the pallet.
			 *
			 * This is set to v2.0.0 for new networks.
			 **/
			storageVersion: AugmentedQuery<ApiType, () => Observable<PalletBalancesReleases>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * The total units issued in the system.
			 **/
			totalIssuance: AugmentedQuery<ApiType, () => Observable<u128>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		grandpa: {
			/**
			 * The number of changes (both in terms of keys and underlying economic responsibilities)
			 * in the "set" of Grandpa validators from genesis.
			 **/
			currentSetId: AugmentedQuery<ApiType, () => Observable<u64>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * next block number where we can force a change.
			 **/
			nextForced: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Pending change: (signaled at, scheduled change).
			 **/
			pendingChange: AugmentedQuery<
				ApiType,
				() => Observable<Option<PalletGrandpaStoredPendingChange>>,
				[]
			> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * A mapping from grandpa set ID to the index of the *most recent* session for which its
			 * members were responsible.
			 *
			 * TWOX-NOTE: `SetId` is not under user control.
			 **/
			setIdSession: AugmentedQuery<
				ApiType,
				(arg: u64 | AnyNumber | Uint8Array) => Observable<Option<u32>>,
				[u64]
			> &
				QueryableStorageEntry<ApiType, [u64]>;
			/**
			 * `true` if we are currently stalled.
			 **/
			stalled: AugmentedQuery<ApiType, () => Observable<Option<ITuple<[u32, u32]>>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * State of the current authority set.
			 **/
			state: AugmentedQuery<ApiType, () => Observable<PalletGrandpaStoredState>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		randomnessCollectiveFlip: {
			/**
			 * Series of block headers from the last 81 blocks that acts as random seed material. This
			 * is arranged as a ring buffer with `block_number % 81` being the index into the `Vec` of
			 * the oldest hash.
			 **/
			randomMaterial: AugmentedQuery<ApiType, () => Observable<Vec<H256>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		sudo: {
			/**
			 * The `AccountId` of the sudo key.
			 **/
			key: AugmentedQuery<ApiType, () => Observable<Option<AccountId32>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		system: {
			/**
			 * The full account information for a particular account ID.
			 **/
			account: AugmentedQuery<
				ApiType,
				(arg: AccountId32 | string | Uint8Array) => Observable<FrameSystemAccountInfo>,
				[AccountId32]
			> &
				QueryableStorageEntry<ApiType, [AccountId32]>;
			/**
			 * Total length (in bytes) for all extrinsics put together, for the current block.
			 **/
			allExtrinsicsLen: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Map of block numbers to block hashes.
			 **/
			blockHash: AugmentedQuery<
				ApiType,
				(arg: u32 | AnyNumber | Uint8Array) => Observable<H256>,
				[u32]
			> &
				QueryableStorageEntry<ApiType, [u32]>;
			/**
			 * The current weight for the block.
			 **/
			blockWeight: AugmentedQuery<
				ApiType,
				() => Observable<FrameSupportWeightsPerDispatchClassU64>,
				[]
			> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Digest of the current block, also part of the block header.
			 **/
			digest: AugmentedQuery<ApiType, () => Observable<SpRuntimeDigest>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * The number of events in the `Events<T>` list.
			 **/
			eventCount: AugmentedQuery<ApiType, () => Observable<u32>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Events deposited for the current block.
			 *
			 * NOTE: This storage item is explicitly unbounded since it is never intended to be read
			 * from within the runtime.
			 **/
			events: AugmentedQuery<ApiType, () => Observable<Vec<FrameSystemEventRecord>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Mapping between a topic (represented by T::Hash) and a vector of indexes
			 * of events in the `<Events<T>>` list.
			 *
			 * All topic vectors have deterministic storage locations depending on the topic. This
			 * allows light-clients to leverage the changes trie storage tracking mechanism and
			 * in case of changes fetch the list of events of interest.
			 *
			 * The value has the type `(T::BlockNumber, EventIndex)` because if we used only just
			 * the `EventIndex` then in case if the topic has the same contents on the next block
			 * no notification will be triggered thus the event might be lost.
			 **/
			eventTopics: AugmentedQuery<
				ApiType,
				(arg: H256 | string | Uint8Array) => Observable<Vec<ITuple<[u32, u32]>>>,
				[H256]
			> &
				QueryableStorageEntry<ApiType, [H256]>;
			/**
			 * The execution phase of the block.
			 **/
			executionPhase: AugmentedQuery<
				ApiType,
				() => Observable<Option<FrameSystemPhase>>,
				[]
			> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Total extrinsics count for the current block.
			 **/
			extrinsicCount: AugmentedQuery<ApiType, () => Observable<Option<u32>>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Extrinsics data for the current block (maps an extrinsic's index to its data).
			 **/
			extrinsicData: AugmentedQuery<
				ApiType,
				(arg: u32 | AnyNumber | Uint8Array) => Observable<Bytes>,
				[u32]
			> &
				QueryableStorageEntry<ApiType, [u32]>;
			/**
			 * Stores the `spec_version` and `spec_name` of when the last runtime upgrade happened.
			 **/
			lastRuntimeUpgrade: AugmentedQuery<
				ApiType,
				() => Observable<Option<FrameSystemLastRuntimeUpgradeInfo>>,
				[]
			> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * The current block number being processed. Set by `execute_block`.
			 **/
			number: AugmentedQuery<ApiType, () => Observable<u32>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Hash of the previous block.
			 **/
			parentHash: AugmentedQuery<ApiType, () => Observable<H256>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * True if we have upgraded so that AccountInfo contains three types of `RefCount`. False
			 * (default) if not.
			 **/
			upgradedToTripleRefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * True if we have upgraded so that `type RefCount` is `u32`. False (default) if not.
			 **/
			upgradedToU32RefCount: AugmentedQuery<ApiType, () => Observable<bool>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		timestamp: {
			/**
			 * Did the timestamp get updated in this block?
			 **/
			didUpdate: AugmentedQuery<ApiType, () => Observable<bool>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Current time for the current block.
			 **/
			now: AugmentedQuery<ApiType, () => Observable<u64>, []> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		transactionPayment: {
			nextFeeMultiplier: AugmentedQuery<ApiType, () => Observable<u128>, []> &
				QueryableStorageEntry<ApiType, []>;
			storageVersion: AugmentedQuery<
				ApiType,
				() => Observable<PalletTransactionPaymentReleases>,
				[]
			> &
				QueryableStorageEntry<ApiType, []>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
		uniques: {
			/**
			 * The assets held by any given account; set out this way so that assets owned by a single
			 * account can be enumerated.
			 **/
			account: AugmentedQuery<
				ApiType,
				(
					arg1: AccountId32 | string | Uint8Array,
					arg2: u64 | AnyNumber | Uint8Array,
					arg3: u32 | AnyNumber | Uint8Array
				) => Observable<Option<Null>>,
				[AccountId32, u64, u32]
			> &
				QueryableStorageEntry<ApiType, [AccountId32, u64, u32]>;
			/**
			 * The assets in existence and their ownership details.
			 **/
			asset: AugmentedQuery<
				ApiType,
				(
					arg1: u64 | AnyNumber | Uint8Array,
					arg2: u32 | AnyNumber | Uint8Array
				) => Observable<Option<PalletUniquesInstanceDetails>>,
				[u64, u32]
			> &
				QueryableStorageEntry<ApiType, [u64, u32]>;
			/**
			 * Metadata of an asset class.
			 **/
			attribute: AugmentedQuery<
				ApiType,
				(
					arg1: u64 | AnyNumber | Uint8Array,
					arg2: Option<u32> | null | object | string | Uint8Array,
					arg3: Bytes | string | Uint8Array
				) => Observable<Option<ITuple<[Bytes, u128]>>>,
				[u64, Option<u32>, Bytes]
			> &
				QueryableStorageEntry<ApiType, [u64, Option<u32>, Bytes]>;
			/**
			 * Details of an asset class.
			 **/
			class: AugmentedQuery<
				ApiType,
				(
					arg: u64 | AnyNumber | Uint8Array
				) => Observable<Option<PalletUniquesClassDetails>>,
				[u64]
			> &
				QueryableStorageEntry<ApiType, [u64]>;
			/**
			 * The classes owned by any given account; set out this way so that classes owned by a single
			 * account can be enumerated.
			 **/
			classAccount: AugmentedQuery<
				ApiType,
				(
					arg1: AccountId32 | string | Uint8Array,
					arg2: u64 | AnyNumber | Uint8Array
				) => Observable<Option<Null>>,
				[AccountId32, u64]
			> &
				QueryableStorageEntry<ApiType, [AccountId32, u64]>;
			/**
			 * Metadata of an asset class.
			 **/
			classMetadataOf: AugmentedQuery<
				ApiType,
				(
					arg: u64 | AnyNumber | Uint8Array
				) => Observable<Option<PalletUniquesClassMetadata>>,
				[u64]
			> &
				QueryableStorageEntry<ApiType, [u64]>;
			/**
			 * Metadata of an asset instance.
			 **/
			instanceMetadataOf: AugmentedQuery<
				ApiType,
				(
					arg1: u64 | AnyNumber | Uint8Array,
					arg2: u32 | AnyNumber | Uint8Array
				) => Observable<Option<PalletUniquesInstanceMetadata>>,
				[u64, u32]
			> &
				QueryableStorageEntry<ApiType, [u64, u32]>;
			/**
			 * Generic query
			 **/
			[key: string]: QueryableStorageEntry<ApiType>;
		};
	} // AugmentedQueries
} // declare module
