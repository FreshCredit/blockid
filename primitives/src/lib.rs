#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::H256;

pub type Balance = u128;
pub type CreditId = u64;
pub type InstanceId = u32;

pub const CARD_HASH_KEY: &str = "fresh_credit_nft";

/// A reference to a piece of data stored off-chain.
///
/// Internally this is a 256-bit BLAKE2 hash of the data, which can be used to retrieve and verify the data
/// from an off-chain source, such as cloud storage or a node, without requiring the data
/// itself to be stored on-chain.
#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct OffchainRef {
  pub hash: H256,
}

#[derive(Clone, Copy, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct CardId {
  pub credit_id: CreditId,
  pub instance_id: InstanceId,
}

/// A registered credit.
#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct Credit<AccountId> {
  /// Credit's full name
  pub name: Vec<u8>,
  /// Account owned by the credit.
  /// Can be reset by root authority if necessary.
  pub credit_account: AccountId,
  /// Credit's kind. Can be modified if necessary.
  pub creditKind: Vec<u8>,
  /// Credit's photo. Can be updated if necessary.
  pub photo: Option<OffchainRef>,
  /// Whether cards have been minted for this credit.
  pub cards_minted: bool,
}

#[derive(Clone, Debug, PartialEq, Decode, Encode, TypeInfo)]
pub struct Card<AccountId, Balance> {
  pub owner: Option<AccountId>,
  pub id: CardId,
  pub tier: CreditCardClass,
  pub value: Balance,
  pub is_on_market: bool,
}

/// Tier of credit card.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Decode, Encode, TypeInfo)]
#[repr(u8)]
pub enum CreditCardClass {
  /// Common tier.
  Gold,
  /// Middle tier.
  Platinum,
  /// Rarest tier.
  Diamond,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Decode, Encode, TypeInfo)]
pub struct InitialCardValues<Balance> {
  pub gold: Balance,
  pub platinum: Balance,
  pub diamond: Balance,
}

/// An application to register a person as a verified credit.
#[derive(Clone, Debug, PartialEq, Decode, Encode, scale_info::TypeInfo)]
pub struct CreditApplication<AccountId> {
  /// Applicant's name
  pub name: Vec<u8>,
  /// Account owned by the applicant
  pub applicant_account: AccountId,
  /// Applicant's kind
  pub creditKind: Vec<u8>,
  /// Optional photo submitted during the application.
  /// The photo can be set or updated after registration, so this isn't mandatory.
  pub photo: Option<OffchainRef>,
}

impl<T> From<CreditApplication<T>> for Credit<T> {
  fn from(other: CreditApplication<T>) -> Self {
    Credit {
      name: other.name,
      credit_account: other.applicant_account,
      creditKind: other.creditKind,
      photo: other.photo,
      cards_minted: false,
    }
  }
}

impl From<CreditCardClass> for u8 {
  fn from(from: CreditCardClass) -> Self {
    from as _
  }
}

pub fn card_hash(credit_id: CreditId, instance_id: InstanceId) -> [u8; 32] {
  blake2_rfc::blake2b::blake2b(
    32,
    CARD_HASH_KEY.as_bytes(),
    &(CardId { credit_id, instance_id }).encode(),
  )
  .as_bytes()
  .try_into()
  .expect("must always be 32 bytes")
}
