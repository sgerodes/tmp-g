#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;


#[frame_support::pallet(dev_mode)]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

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

        #[pallet::call_index(1)]
        #[pallet::weight(5_000)]
        pub fn get(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let data = UserStrings::<T>::get(&who);
            Self::deposit_event(Event::StringGet(who, data));
            Ok(())
        }

    }

}