use codec::{Decode, Encode, MaxEncodedLen};
#[cfg(feature = "runtime-benchmarks")]
use enumflags2::BitFlag;
use enumflags2::{bitflags, BitFlags};
use frame_support::{traits::Get, CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use pallet_identity::{Data, IdentityInformationProvider};
use scale_info::{build::Variants, Path, Type, TypeInfo};
use sp_runtime::{BoundedVec, RuntimeDebug};
use sp_std::prelude::*;

/// The fields that we use to identify the owner of an account with. Each corresponds to a field
/// in the `IdentityInfo` struct.
#[bitflags]
#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug)]
pub enum IdentityField {
    Display,
    FirstName,
    LastName,
    Email,
    Address,
    TelephoneNumber,
    Bio,
}

impl TypeInfo for IdentityField {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        Type::builder()
            .path(Path::new("IdentityField", module_path!()))
            .variant(
                Variants::new()
                    .variant("Display", |v| v.index(0))
                    .variant("FirstName", |v| v.index(1))
                    .variant("LastName", |v| v.index(2))
                    .variant("Email", |v| v.index(3))
                    .variant("Address", |v| v.index(4))
                    .variant("TelephoneNumber", |v| v.index(5))
                    .variant("Bio", |v| v.index(6)),
            )
    }
}

/// Information concerning the identity of the controller of an account.
///
/// NOTE: This should be stored at the end of the storage item to facilitate the addition of extra
/// fields in a backwards compatible way through a specialized `Decode` impl.
#[derive(
    CloneNoBound,
    Encode,
    Decode,
    EqNoBound,
    MaxEncodedLen,
    PartialEqNoBound,
    RuntimeDebugNoBound,
    TypeInfo,
)]
#[codec(mel_bound())]
#[scale_info(skip_type_params(FieldLimit))]
pub struct IdentityInfo<FieldLimit: Get<u32>> {
    /// Additional fields of the identity that are not catered for with the struct's explicit
    /// fields.
    pub additional: BoundedVec<(Data, Data), FieldLimit>,

    /// A reasonable display name for the controller of the account. This should be whatever it is
    /// that it is typically known as and should not be confusable with other entities, given
    /// reasonable context.
    ///
    /// Stored as UTF-8.
    pub display: Data,

    /// The full legal name in the local jurisdiction of the entity. This might be a bit
    /// long-winded.
    ///
    /// Stored as UTF-8.
    // pub legal: Data,
    pub first_name: Data,
    pub last_name: Data,

    /// The email address of the controller of the account.
    ///
    /// Stored as UTF-8.
    pub email: Data,

    pub address: Data,

    pub telephone_number: Data,

    pub bio: Data,
}

impl<FieldLimit: Get<u32> + 'static> IdentityInformationProvider for IdentityInfo<FieldLimit> {
    type FieldsIdentifier = u64;

    fn has_identity(&self, fields: Self::FieldsIdentifier) -> bool {
        self.fields().bits() & fields == fields
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn create_identity_info() -> Self {
        let data = Data::Raw(vec![0; 32].try_into().unwrap());

        IdentityInfo {
            additional: vec![(data.clone(), data.clone()); FieldLimit::get().try_into().unwrap()]
                .try_into()
                .unwrap(),
            display: data.clone(),
            first_name: data.clone(),
            last_name: data.clone(),
            email: data.clone(),
            address: data.clone(),
            telephone_number: data.clone(),
            bio: data.clone(),
        }
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn all_fields() -> Self::FieldsIdentifier {
        IdentityField::all().bits()
    }
}

impl<FieldLimit: Get<u32>> Default for IdentityInfo<FieldLimit> {
    fn default() -> Self {
        IdentityInfo {
            additional: BoundedVec::default(),
            display: Data::None,
            first_name: Data::None,
            last_name: Data::None,
            email: Data::None,
            address: Data::None,
            telephone_number: Data::None,
            bio: Data::None,
        }
    }
}

impl<FieldLimit: Get<u32>> IdentityInfo<FieldLimit> {
    pub(crate) fn fields(&self) -> BitFlags<IdentityField> {
        let mut res = <BitFlags<IdentityField>>::empty();
        if !self.display.is_none() {
            res.insert(IdentityField::Display);
        }
        if !self.first_name.is_none() {
            res.insert(IdentityField::FirstName);
        }
        if !self.last_name.is_none() {
            res.insert(IdentityField::LastName);
        }
        if !self.email.is_none() {
            res.insert(IdentityField::Email);
        }
        if !self.address.is_none() {
            res.insert(IdentityField::Address);
        }
        if !self.telephone_number.is_none() {
            res.insert(IdentityField::TelephoneNumber);
        }
        if !self.bio.is_none() {
            res.insert(IdentityField::Bio);
        }
        res
    }
}

// use frame_support::pallet_prelude::BuildGenesisConfig;
// use sp_runtime::traits::StaticLookup;
// use sp_runtime::serde::{Deserialize, Serialize};
//
// #[derive(Default, Serialize, Deserialize)]
// #[serde(bound = "T: frame_system::Config")]
// pub struct MyPalletGenesisConfig<T: frame_system::Config> {
//     pub registrars: Vec<T::AccountId>,
// }
//
// impl<T: pallet_identity::Config> BuildGenesisConfig for MyPalletGenesisConfig<T> {
//     fn build(&self) {
//         for registrar in &self.registrars {
//             let registrar_lookup = T::Lookup::unlookup(registrar.clone());
//             pallet_identity::Pallet::<T>::add_registrar(frame_system::RawOrigin::Root.into(), registrar_lookup)
//                 .expect("Failed to add registrar");
//         }
//     }
// }

// use frame_support::pallet_prelude::BuildGenesisConfig;
// use frame_support::traits::GenesisBuild;
// // use serde::{Deserialize, Serialize};
// use sp_runtime::serde::{Deserialize, Serialize};
// use sp_runtime::traits::StaticLookup;
//
// const SEED: u32 = 0;
//
// #[derive(Default, Serialize, Deserialize)]
// #[serde(bound = "T: frame_system::Config")]
// pub struct IdentityGenesisConfig<T: frame_system::Config> {
//     pub registrars: Vec<T::AccountId>,
// }
//
//
// impl<T: pallet_identity::Config> BuildGenesisConfig for IdentityGenesisConfig<T> {
//     fn build(&self) {
//         for registrar in &self.registrars {
//             let registrar_lookup = T::Lookup::unlookup(registrar.clone());
//             Identity::<T>::add_registrar(frame_system::RawOrigin::Root.into(), registrar_lookup).expect("Failed to add registrar");
//
//             // pallet_identity::Pallet::<T>::add_registrar(frame_system::RawOrigin::Root.into(), registrar).expect("Failed to add registrar");
//         }
//     }
// }

// impl<T: pallet_identity::Config> GenesisBuild<T> for IdentityGenesisConfig {
//     fn build(&self) {
//         for registrar in &self.registrars {
//             pallet_identity::Pallet::<T>::add_registrar(frame_system::RawOrigin::Root.into(), registrar.clone());
//         }
//     }
// }

// impl pallet_identity::GenesisConfig<Runtime> for IdentityGenesisConfig {
//     fn build(&self) {
//         for (account_id, registrar) in &self.registrars {
//             // pallet_identity::Registrars::<Runtime>::append(registrar.clone());
//             // // Here we assume registrar ID starts from 0 and increments by 1 for each registrar
//             // let registrar_count = pallet_identity::Registrars::<Runtime>::decode_len().unwrap_or(0);
//             // let registrar_index = registrar_count as u32;
//             // pallet_identity::Pallet::<Runtime>::add_registrar(account_id.clone(), registrar_index);
//             pallet_identity::add_registrar()
//         }
//     }
// }

// #[benchmark]
// fn reap_identity(
//     r: Linear<0, { T::MaxRegistrars::get() }>,
//     s: Linear<0, { T::MaxSubAccounts::get() }>,
// ) -> Result<(), BenchmarkError> {
//     // set up target
//     let target: T::AccountId = account("target", 0, SEED);
//     let target_origin =
//         <T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(target.clone()));
//     let target_lookup = T::Lookup::unlookup(target.clone());
//     let _ = T::Currency::make_free_balance_be(&target, BalanceOf::<T>::max_value());
//
//     // set identity
//     let info = <T as pallet_identity::Config>::IdentityInformation::create_identity_info();
//     Identity::<T>::set_identity(
//         RawOrigin::Signed(target.clone()).into(),
//         Box::new(info.clone()),
//     )?;
//
//     // create and set subs
//     let mut subs = Vec::new();
//     let data = Data::Raw(vec![0; 32].try_into().unwrap());
//     for ii in 0..s {
//         let sub_account = account("sub", ii, SEED);
//         subs.push((sub_account, data.clone()));
//     }
//     Identity::<T>::set_subs(target_origin.clone(), subs.clone())?;
//
//     // add registrars and provide judgements
//     let registrar_origin = T::RegistrarOrigin::try_successful_origin()
//         .expect("RegistrarOrigin has no successful origin required for the benchmark");
//     for ii in 0..r {
//         // registrar account
//         let registrar: T::AccountId = account("registrar", ii, SEED);
//         let registrar_lookup = T::Lookup::unlookup(registrar.clone());
//         let _ = <T as pallet_identity::Config>::Currency::make_free_balance_be(
//             &registrar,
//             <T as pallet_identity::Config>::Currency::minimum_balance(),
//         );
//
//         // add registrar
//         Identity::<T>::add_registrar(registrar_origin.clone(), registrar_lookup)?;
//         Identity::<T>::set_fee(RawOrigin::Signed(registrar.clone()).into(), ii, 10u32.into())?;
//         let fields = <T as pallet_identity::Config>::IdentityInformation::all_fields();
//         Identity::<T>::set_fields(RawOrigin::Signed(registrar.clone()).into(), ii, fields)?;
//
//         // request and provide judgement
//         Identity::<T>::request_judgement(target_origin.clone(), ii, 10u32.into())?;
//         Identity::<T>::provide_judgement(
//             RawOrigin::Signed(registrar).into(),
//             ii,
//             target_lookup.clone(),
//             Judgement::Reasonable,
//             <T as frame_system::Config>::Hashing::hash_of(&info),
//         )?;
//     }
//
//     let origin = T::Reaper::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
//
//     #[extrinsic_call]
//     _(origin as T::RuntimeOrigin, target.clone());
//
//     assert_last_event::<T>(Event::<T>::IdentityReaped { who: target.clone() }.into());
//
//     let fields = <T as pallet_identity::Config>::IdentityInformation::all_fields();
//     assert!(!Identity::<T>::has_identity(&target, fields));
//     assert_eq!(Identity::<T>::subs(&target).len(), 0);
//
//     Ok(())
// }
