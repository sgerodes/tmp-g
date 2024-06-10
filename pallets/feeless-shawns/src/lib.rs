#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        // dispatch::{DispatchResult, Dispatchable, DispatchResultWithPostInfo},
        dispatch::{DispatchResult, DispatchResultWithPostInfo},
        pallet_prelude::*,
        Parameter,
        // weights::GetDispatchInfo,
        traits::Get,
        sp_runtime::traits::BlockNumberProvider,

    };
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::boxed::Box;

    use frame_support::dispatch::GetDispatchInfo;
    use sp_runtime::traits::Dispatchable;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        // type RuntimeCall: Parameter + GetDispatchInfo + Dispatchable<Origin=Self::Origin>;
        type RuntimeCall: Parameter + GetDispatchInfo + Dispatchable;

    }

    #[pallet::error]
    pub enum Error<T> {
        ExceedMaxCalls,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        YES
    }


    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight({
            let dispatch_info = call.get_dispatch_info();
            (dispatch_info.weight, dispatch_info.class, Pays::Yes)
        })]
        pub fn make_feeless(
            origin: OriginFor<T>,
            call: Box<<T as Config>::RuntimeCall>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin.clone())?;
            return Ok(Pays::No.into())
        }
    }
}
