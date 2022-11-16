#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

pub mod nft;
use sp_runtime::traits::{StaticLookup};


#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Maximum offchain data length.
		#[pallet::constant]
		type NFTOffchainDataLimit: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Counter for NFT ids.
	#[pallet::storage]
	#[pallet::getter(fn next_nft_id)]
	pub type NextNFTId<T: Config> = StorageValue<_, crate::nft::NFTId, ValueQuery>;

	/// Data related to NFTs.
	#[pallet::storage]
	#[pallet::getter(fn nfts)]
	pub type Nfts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		crate::nft::NFTId,
		crate::nft::NFTData<T::AccountId, T::NFTOffchainDataLimit>,
		OptionQuery,
	>;

	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A new NFT was created.
		NFTCreated {
			nft_id: crate::nft::NFTId,
			owner: T::AccountId,
			offchain_data: crate::nft::U8BoundedVec<T::NFTOffchainDataLimit>,
		},
		/// An NFT was burned.
		NFTBurned { nft_id: crate::nft::NFTId },
		/// An NFT was transferred to someone else.
		NFTTransferred { nft_id: crate::nft::NFTId, sender: T::AccountId, recipient: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// No NFT was found with that NFT id.
		NFTNotFound,
		/// This function can only be called by the owner of the NFT.
		NotTheNFTOwner,
		/// Operation is not allowed because the NFT is owned by the caller.
		CannotTransferNFTsToYourself,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/// Create a new NFT with the provided details. An ID will be auto
		/// generated and logged as an event, The caller of this function
		/// will become the owner of the new NFT.
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn create_nft(
			origin: OriginFor<T>,
			offchain_data: crate::nft::U8BoundedVec<T::NFTOffchainDataLimit>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let mut next_nft_id = None;

			let nft_id = next_nft_id.unwrap_or_else(|| Self::get_next_nft_id());
			let nft = crate::nft::NFTData::new_default(
				who.clone(),
				offchain_data.clone(),
			);
			// Execute
			Nfts::<T>::insert(nft_id, nft);
			let event = Event::NFTCreated {
				nft_id,
				owner: who,
				offchain_data,

			};
			Self::deposit_event(event);

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn burn_nft(origin: OriginFor<T>, nft_id: crate::nft::NFTId) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let nft = Nfts::<T>::get(nft_id).ok_or(Error::<T>::NFTNotFound)?;

			// Checks
			ensure!(nft.owner == who, Error::<T>::NotTheNFTOwner);
			// Execute
			Nfts::<T>::remove(nft_id);
			Self::deposit_event(Event::NFTBurned { nft_id });

			Ok(().into())
		}

		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(1))]
		pub fn transfer_nft(
			origin: OriginFor<T>,
			nft_id: crate::nft::NFTId,
			recipient: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let recipient = T::Lookup::lookup(recipient)?;

			Nfts::<T>::try_mutate(nft_id, |x| -> DispatchResult {
				let nft = x.as_mut().ok_or(Error::<T>::NFTNotFound)?;

				// Checks
				ensure!(nft.owner == who, Error::<T>::NotTheNFTOwner);
				ensure!(nft.owner != recipient, Error::<T>::CannotTransferNFTsToYourself);
				// Execute
				nft.owner = recipient.clone();

				Ok(().into())
			})?;
			// Execute
			let event = Event::NFTTransferred { nft_id, sender: who, recipient };
			Self::deposit_event(event);

			Ok(().into())
		}

	}

	impl<T: Config> Pallet<T> {
		fn get_next_nft_id() -> crate::nft::NFTId {
			let nft_id = NextNFTId::<T>::get();
			let next_id = nft_id
				.checked_add(1)
				.expect("If u32 is not enough we should crash for safety; qed.");
			NextNFTId::<T>::put(next_id);
	
			nft_id
		}
	}
}
