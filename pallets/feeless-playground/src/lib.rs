#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use frame_support::dispatch::GetDispatchInfo;
use frame_support::traits::UnfilteredDispatchable;
// use frame_system::RawOrigin;
use scale_info::prelude::boxed::Box;
use frame_support::dispatch::PostDispatchInfo;
use sp_runtime::traits::Dispatchable;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;


// #[frame_support::pallet(dev_mode)]
#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // #[pallet::no_default_bounds]
        // type RuntimeCall: Parameter + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin> + GetDispatchInfo;
        type RuntimeCall: Parameter
        + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
        + GetDispatchInfo
        + Dispatchable<
            RuntimeOrigin = Self::RuntimeOrigin,
            PostInfo = PostDispatchInfo,
        >;
        type WeightInfo: WeightInfo;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DummyEvent,
        FeelessTransaction
    }

    #[pallet::error]
    pub enum Error<T> {
        DummyError // TODO change this
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::feeless_fiesta())]
        pub fn feeless_fiesta(
            origin: OriginFor<T>,
            call: Box<<T as Config>::RuntimeCall>,
        ) -> DispatchResultWithPostInfo {
            let _sender = ensure_signed(origin.clone())?;

            // let _res = call.dispatch_bypass_filter(origin);
            let _res = call.dispatch(origin);

            Self::deposit_event(Event::FeelessTransaction);
            Ok(Pays::No.into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn do_something(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Self::deposit_event(Event::DummyEvent);
            Ok(())
        }
    }

}