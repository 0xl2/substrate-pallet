#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
  	use frame_system::pallet_prelude::*;
  	use sp_std::vec::Vec;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a novel has been created. [who, claim]
		NovelCreated(T::AccountId, Vec<u8>),
		/// Event emitted when a novel is read. [who, claim]
		ReadNovel(T::AccountId),
		/// Event emitted when a novel is updated by the owner. [who, claim]
		NovelUpdated(T::AccountId, Vec<u8>),
		/// Event emitted when a novel is removed by the owner. [who, claim]
		NovelRemoved(T::AccountId, Vec<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The proof has already been claimed.
		NovelAlreadyClaimed,
		/// The novel does not exist, so it cannot be revoked.
		NoSuchNovel,
		/// The novel is claimed by another account, so caller can't revoke it.
		NotNovelOwner,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);
	  
	#[pallet::storage]
	pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_novel(
			origin: OriginFor<T>,
			proof: Vec<u8>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			
			// Verify that the specified proof has not already been claimed.
			ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::NovelAlreadyClaimed);
			
			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();
			
			// Store the proof with the sender and block number.
			Proofs::<T>::insert(&proof, (&sender, current_block));
			
			// Emit an event that the claim was created.
			Self::deposit_event(Event::NovelCreated(sender, proof));
			
			Ok(())
		}

		#[pallet::weight(20_000)]
		pub fn read_novel(
			origin: OriginFor<T>,
			proof: Vec<u8>,
        ) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchNovel);

			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotNovelOwner);

			

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ReadNovel(sender));
			Ok(())
        }

		#[pallet::weight(30_000)]
		pub fn update_novel(
			origin: OriginFor<T>,
			proof: Vec<u8>,
        ) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchNovel);

			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);
			
			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotNovelOwner);

			Proofs::<T>::remove(&proof);
			let current_block = <frame_system::Pallet<T>>::block_number();
			Proofs::<T>::insert(&proof, (&sender, current_block));

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::NovelUpdated(sender, proof));
			Ok(())
        }
      
		#[pallet::weight(40_000)]
		pub fn remove_novel(
			origin: OriginFor<T>,
			proof: Vec<u8>,
        ) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			
			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchNovel);
			
			// Get owner of the claim.
			let (owner, _) = Proofs::<T>::get(&proof);
			
			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotNovelOwner);
			
			// Remove claim from storage.
			Proofs::<T>::remove(&proof);
			
			// Emit an event that the claim was erased.
			Self::deposit_event(Event::NovelRemoved(sender, proof));
			Ok(())
        }
  	}

}
