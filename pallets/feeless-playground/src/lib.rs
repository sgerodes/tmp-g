#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch::GetDispatchInfo;
use frame_support::traits::UnfilteredDispatchable;
pub use pallet::*;
// use frame_system::RawOrigin;
use frame_support::dispatch::PostDispatchInfo;
use scale_info::prelude::boxed::Box;
use sp_runtime::traits::Dispatchable;
// use frame_support::dispatch::Dispatchable;

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

    // #[pallet::config]
    #[pallet::config(with_default)]
    pub trait Config: frame_system::Config {
        #[pallet::no_default_bounds]
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        #[pallet::no_default_bounds]
        type RuntimeCall: Parameter
            + Dispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + From<Call<Self>>;
        type WeightInfo: WeightInfo;

        // #[pallet::no_default_bounds]
        // type RuntimeEvent: Parameter
        // + Member
        // + From<Event<Self>>
        // + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        // #[pallet::no_default_bounds]
        // type RuntimeCall: Parameter
        // + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
        // + GetDispatchInfo;

        // type RuntimeCall: Parameter
        // + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
        // + GetDispatchInfo
        // + Dispatchable<
        //     RuntimeOrigin = Self::RuntimeOrigin,
        //     PostInfo = PostDispatchInfo,
        // >;
        // type RuntimeCall: Parameter + GetDispatchInfo + Dispatchable<Origin=Self::RuntimeOrigin>;
        // type RuntimeOrigin: From<RuntimeOrigin> + From<<Self as frame_system::Config>::RuntimeOrigin>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DummyEvent,
        FeelessTransaction,
    }

    #[pallet::error]
    pub enum Error<T> {
        DummyError, // TODO change this
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::feeless_fiesta())]
        // #[pallet::weight({
        //     let dispatch_info = call.get_dispatch_info();
        //     (dispatch_info.weight.saturating_add(10_000.into()), dispatch_info.class, Pays::Yes)
        // })]
        pub fn feeless_fiesta(
            origin: OriginFor<T>,
            // origin: T::RuntimeOrigin,
            call: Box<<T as Config>::RuntimeCall>,
            // call: <T as pallet::Config>::RuntimeCall,
        ) -> DispatchResultWithPostInfo {
            // let _sender = ensure_signed(origin.clone())?;

            // let _res = call.dispatch_bypass_filter(origin).map_err(|e| e.error)?;
            let _res = call.dispatch(origin);

            Self::deposit_event(Event::FeelessTransaction);
            Ok(Pays::No.into())
        }

        #[pallet::call_index(1)]
        // #[pallet::weight(100)]
        #[pallet::weight(T::WeightInfo::do_something())]
        pub fn do_something(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            Self::deposit_event(Event::DummyEvent);
            Ok(())
        }
    }
}
