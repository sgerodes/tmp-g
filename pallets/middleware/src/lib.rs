#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

use sp_runtime::offchain::storage::StorageValueRef;
use frame_system::pallet_prelude::*;
use core::convert::TryInto;
use sp_core::offchain::StorageKind;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    // use frame_support::{
    //     dispatch::{DispatchResult, DispatchResultWithPostInfo},
    // };

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn user_strings)]
    pub type UserStrings<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u8, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        StringSet(T::AccountId, u8),
        StringGet(T::AccountId, u8),
    }

    #[pallet::error]
    pub enum Error<T> {
        StringTooLong,
    }

    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    //     // #[cfg(feature = "experimental")]
    //     fn offchain_worker(block_number: BlockNumberFor<T>) {
    //         let block_number_u64: u64 = block_number.try_into().ok().unwrap_or_default();
    //         println!("Current print block number: {:?}", block_number_u64);
    //         log::debug!("Current log block number: {:?}", block_number_u64);
    //     }
    // }

    // #[pallet::hooks]
    // impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    //     /// Offchain Worker entry point.
    //     ///
    //     /// By implementing `fn offchain_worker` you declare a new offchain worker.
    //     /// This function will be called when the node is fully synced and a new best block is
    //     /// successfully imported.
    //     /// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
    //     /// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
    //     /// so the code should be able to handle that.
    //     /// You can use `Local Storage` API to coordinate runs of the worker.
    //     fn offchain_worker(block_number: BlockNumberFor<T>) {
    //         // Note that having logs compiled to WASM may cause the size of the blob to increase
    //         // significantly. You can use `RuntimeDebug` custom derive to hide details of the types
    //         // in WASM. The `sp-api` crate also provides a feature `disable-logging` to disable
    //         // all logging and thus, remove any logging from the WASM.
    //         log::info!("Hello World from offchain workers!");
    //
    //         // Since off-chain workers are just part of the runtime code, they have direct access
    //         // to the storage and other included pallets.
    //         //
    //         // We can easily import `frame_system` and retrieve a block hash of the parent block.
    //         let parent_hash = <system::Pallet<T>>::block_hash(block_number - 1u32.into());
    //         log::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);
    //
    //         // It's a good practice to keep `fn offchain_worker()` function minimal, and move most
    //         // of the code to separate `impl` block.
    //         // Here we call a helper function to calculate current average price.
    //         // This function reads storage entries of the current state.
    //         let average: Option<u32> = Self::average_price();
    //         log::debug!("Current price: {:?}", average);
    //
    //         // For this example we are going to send both signed and unsigned transactions
    //         // depending on the block number.
    //         // Usually it's enough to choose one or the other.
    //         let should_send = Self::choose_transaction_type(block_number);
    //         let res = match should_send {
    //             TransactionType::Signed => Self::fetch_price_and_send_signed(),
    //             TransactionType::UnsignedForAny =>
    //                 Self::fetch_price_and_send_unsigned_for_any_account(block_number),
    //             TransactionType::UnsignedForAll =>
    //                 Self::fetch_price_and_send_unsigned_for_all_accounts(block_number),
    //             TransactionType::Raw => Self::fetch_price_and_send_raw_unsigned(block_number),
    //             TransactionType::None => Ok(()),
    //         };
    //         if let Err(e) = res {
    //             log::error!("Error: {}", e);
    //         }
    //     }
    // }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set(origin: OriginFor<T>, user_input: u8) -> DispatchResult {
            let who = ensure_signed(origin)?;
            UserStrings::<T>::insert(&who, user_input.clone());
            Self::deposit_event(Event::StringSet(who, user_input));
            Ok(())
        }
        //
        // #[pallet::call_index(1)]
        // #[pallet::weight(5_000)]
        // pub fn get(origin: OriginFor<T>) -> DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     let data = UserStrings::<T>::get(&who);
        //     Self::deposit_event(Event::StringGet(who, data));
        //     Ok(())
        // }
        //
        // #[pallet::call_index(2)]
        // #[pallet::weight(5_000)]
        // pub fn upload_picture(origin: OriginFor<T>, picture: Vec<u8>) -> DispatchResult {
        //     let who = ensure_signed(origin)?;
        //
        //
        //     Ok(())
        // }
        //
        // /// Submit new price to the list.
        // ///
        // /// This method is a public function of the module and can be called from within
        // /// a transaction. It appends given `price` to current list of prices.
        // /// In our example the `offchain worker` will create, sign & submit a transaction that
        // /// calls this function passing the price.
        // ///
        // /// The transaction needs to be signed (see `ensure_signed`) check, so that the caller
        // /// pays a fee to execute it.
        // /// This makes sure that it's not easy (or rather cheap) to attack the chain by submitting
        // /// excessive transactions, but note that it doesn't ensure the price oracle is actually
        // /// working and receives (and provides) meaningful data.
        // /// This example is not focused on correctness of the oracle itself, but rather its
        // /// purpose is to showcase offchain worker capabilities.
        // #[pallet::call_index(3)]
        // #[pallet::weight({0})]
        // pub fn submit_price(origin: OriginFor<T>, price: u32) -> DispatchResultWithPostInfo {
        //     // Retrieve sender of the transaction.
        //     let who = ensure_signed(origin)?;
        //     // Add the price to the on-chain list.
        //     Self::add_price(Some(who), price);
        //     Ok(().into())
        // }
        //
        // /// Submit new price to the list via unsigned transaction.
        // ///
        // /// Works exactly like the `submit_price` function, but since we allow sending the
        // /// transaction without a signature, and hence without paying any fees,
        // /// we need a way to make sure that only some transactions are accepted.
        // /// This function can be called only once every `T::UnsignedInterval` blocks.
        // /// Transactions that call that function are de-duplicated on the pool level
        // /// via `validate_unsigned` implementation and also are rendered invalid if
        // /// the function has already been called in current "session".
        // ///
        // /// It's important to specify `weight` for unsigned calls as well, because even though
        // /// they don't charge fees, we still don't want a single block to contain unlimited
        // /// number of such transactions.
        // ///
        // /// This example is not focused on correctness of the oracle itself, but rather its
        // /// purpose is to showcase offchain worker capabilities.
        // #[pallet::call_index(4)]
        // #[pallet::weight({0})]
        // pub fn submit_price_unsigned(
        //     origin: OriginFor<T>,
        //     _block_number: BlockNumberFor<T>,
        //     price: u32,
        // ) -> DispatchResultWithPostInfo {
        //     // This ensures that the function can only be called via unsigned transaction.
        //     ensure_none(origin)?;
        //     // Add the price to the on-chain list, but mark it as coming from an empty address.
        //     Self::add_price(None, price);
        //     // now increment the block number at which we expect next unsigned transaction.
        //     let current_block = <system::Pallet<T>>::block_number();
        //     <NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
        //     Ok(().into())
        // }
        //
        // #[pallet::call_index(5)]
        // #[pallet::weight({0})]
        // pub fn submit_price_unsigned_with_signed_payload(
        //     origin: OriginFor<T>,
        //     price_payload: PricePayload<T::Public, BlockNumberFor<T>>,
        //     _signature: T::Signature,
        // ) -> DispatchResultWithPostInfo {
        //     // This ensures that the function can only be called via unsigned transaction.
        //     ensure_none(origin)?;
        //     // Add the price to the on-chain list, but mark it as coming from an empty address.
        //     Self::add_price(None, price_payload.price);
        //     // now increment the block number at which we expect next unsigned transaction.
        //     let current_block = <system::Pallet<T>>::block_number();
        //     <NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());
        //     Ok(().into())
        // }
    }
}
