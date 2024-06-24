#[cfg(feature = "runtime-benchmarks")]
use enumflags2::BitFlag;
use enumflags2::{bitflags, BitFlags};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use scale_info::{build::Variants, Path, Type, TypeInfo};
use sp_runtime::{BoundedVec, RuntimeDebug};
use sp_std::prelude::*;
use pallet_identity::{Data, IdentityInformationProvider};

macro_rules! define_identity_info {
    ($($camel_case:ident => $snake_case:ident),*) => {
        #[bitflags]
        #[repr(u64)]
        #[derive(Clone, Copy, PartialEq, Eq, RuntimeDebug)]
        pub enum IdentityField {
            $(
                $camel_case,
            )*
        }

        impl TypeInfo for IdentityField {
            type Identity = Self;

            fn type_info() -> scale_info::Type {
                Type::builder().path(Path::new("IdentityField", module_path!())).variant(
                    Variants::new()
                        $(
                            .variant(stringify!($camel_case), |v| v.index(IdentityField::$camel_case as u64 as u8))
                        )*
                )
            }
        }

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
                pub additional: BoundedVec<(Data, Data), FieldLimit>,
            $(
                pub $snake_case: Data,
            )*
        }

        // impl<FieldLimit: Get<u32> + 'static> IdentityInformationProvider for IdentityInfo<FieldLimit> {
        //     type FieldsIdentifier = u64;
        //
        //     fn has_identity(&self, fields: Self::FieldsIdentifier) -> bool {
        //         self.fields().bits() & fields == fields
        //     }
        //
        //     #[cfg(feature = "runtime-benchmarks")]
        //     fn create_identity_info() -> Self {
        //         let data = Data::Raw(vec![0; 32].try_into().unwrap());
        //         IdentityInfo {
        //             $(
        //                 $snake_case: data.clone(),
        //             )*
        //         }
        //     }
        //
        //     #[cfg(feature = "runtime-benchmarks")]
        //     fn all_fields() -> Self::FieldsIdentifier {
        //         IdentityField::all().bits()
        //     }
        // }
        //
        // impl<FieldLimit: Get<u32>> Default for IdentityInfo<FieldLimit> {
        //     fn default() -> Self {
        //         IdentityInfo {
        //             $(
        //                 $snake_case: Data::None,
        //             )*
        //         }
        //     }
        // }
        //
        // impl<FieldLimit: Get<u32>> IdentityInfo<FieldLimit> {
        //     pub(crate) fn fields(&self) -> BitFlags<IdentityField> {
        //         let mut res = <BitFlags<IdentityField>>::empty();
        //         $(
        //             if !self.$snake_case.is_none() {
        //                 res.insert(IdentityField::$camel_case);
        //             }
        //         )*
        //         res
        //     }
        // }
    }
}

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
    pub additional: BoundedVec<(Data, Data), FieldLimit>,
    pub fields: IdentityFields,
}

// Usage
define_identity_info!(
    Display => display
    // FirstName => first_name,
    // LastName => last_name,
    // Email => email,
    // Address => address,
    // TelephoneNumber => telephone_number,
    // Bio => bio
);