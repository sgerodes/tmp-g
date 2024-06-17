#[cfg(feature = "runtime-benchmarks")]
use enumflags2::BitFlag;
use enumflags2::{bitflags, BitFlags};
use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{traits::Get, CloneNoBound, EqNoBound, PartialEqNoBound, RuntimeDebugNoBound};
use scale_info::{build::Variants, Path, Type, TypeInfo};
use sp_runtime::{BoundedVec, RuntimeDebug};
use sp_std::prelude::*;
use pallet_identity::{Data, IdentityInformationProvider};


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
}

impl TypeInfo for IdentityField {
    type Identity = Self;

    fn type_info() -> scale_info::Type {
        Type::builder().path(Path::new("IdentityField", module_path!())).variant(
            Variants::new()
                .variant("Display", |v| v.index(0))
                .variant("FirstName", |v| v.index(1))
                .variant("LastName", |v| v.index(2))
                .variant("Email", |v| v.index(3))
                .variant("Address", |v| v.index(4))
                .variant("TelephoneNumber", |v| v.index(5))
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
        res
    }
}

