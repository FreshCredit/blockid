use frame_support::{
	inherent::Vec, traits::Get, BoundedVec, CloneNoBound, PartialEqNoBound, RuntimeDebug,
	RuntimeDebugNoBound,
};
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_arithmetic::per_things::Permill;
use sp_std::fmt::Debug;

/// How NFT IDs are encoded.
pub type NFTId = u32;

pub type U8BoundedVec<S> = BoundedVec<u8, S>;

/// Data related to an NFT, such as who is its owner.
#[derive(
	Encode,
	Decode,
	Eq,
	Default,
	TypeInfo,
	CloneNoBound,
	PartialEqNoBound,
	RuntimeDebugNoBound,
	MaxEncodedLen,
)]
#[scale_info(skip_type_params(NFTOffchainDataLimit))]
#[codec(mel_bound(AccountId: MaxEncodedLen))]
pub struct NFTData<AccountId, NFTOffchainDataLimit>
where
	AccountId: Clone + PartialEq + Debug,
	NFTOffchainDataLimit: Get<u32>,
{
	/// NFT owner
	pub owner: AccountId,
	/// NFT creator
	pub creator: AccountId,
	/// NFT offchain_data
	pub offchain_data: U8BoundedVec<NFTOffchainDataLimit>,
}


impl<AccountId, NFTOffchainDataLimit> NFTData<AccountId, NFTOffchainDataLimit>
where
	AccountId: Clone + PartialEq + Debug,
	NFTOffchainDataLimit: Get<u32>,
{
	pub fn new(
		owner: AccountId,
		creator: AccountId,
		offchain_data: U8BoundedVec<NFTOffchainDataLimit>,
	) -> Self {
		Self { owner, creator, offchain_data }
	}

	pub fn new_default(
		owner: AccountId,
		offchain_data: U8BoundedVec<NFTOffchainDataLimit>,
	) -> Self {
		Self::new(
			owner.clone(),
			owner,
			offchain_data,
		)
	}
}