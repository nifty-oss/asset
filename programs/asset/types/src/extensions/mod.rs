//! Extensions are used to add additional data to an asset.
//!
//! It is possible to attach additional data to an asset using extensions. The `Extension`
//! struct provides the "header" information for the extension, and stores (1) the type of the
//! extension, (2) the length of the extension data, and (3) the boundary of the extension on the
//! account data buffer.
//!
//! The type and length are determined by the extension itself. The boundary is used internally
//! to make sure that each extensions data is aligned to a 8-byte boundary. This is required to
//! support extensions that store their data using bytemuck's `Pod` trait.
//!
//! Note that is it possible to have extensions without any data, i.e., no `ExtensionData`.
//! In this case, the extension is a "marker" – a particular behaviour can be derived by the
//! presence/absence of the extension.

mod attributes;
mod blob;
mod creators;
mod grouping;
mod links;
mod manager;
mod metadata;
mod proxy;
mod royalties;

pub use attributes::*;
pub use blob::*;
pub use creators::*;
pub use grouping::*;
pub use links::*;
pub use manager::*;
pub use metadata::*;
pub use proxy::*;
pub use royalties::*;

use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;
use std::{fmt::Debug, ops::Deref};

use crate::{error::Error, state::Asset};

/// The `Extension` struct is used to store the "header" information for an extension.
///
/// This information is added at the start of each extension data and it is used to determine
/// the type of extension and the length of the extension data.
#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct Extension {
    /// Data section.
    ///   0. type
    ///   1. length
    ///   2. boundary
    ///   3. unused
    data: [u32; 4],
}

impl Extension {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub fn new(extension_type: ExtensionType, length: u32, boundary: u32) -> Self {
        Self {
            data: [extension_type.into(), length, boundary, 0],
        }
    }

    /// Try to get the extension type.
    ///
    /// The extension type is stored as a `u32` on the acccount data. This method tries to
    /// perform the conversion to an `ExtensionType` and returns an error if the conversion
    /// fails – e.g., the `u32` value is not a valid extension type. This can happen when
    /// a new extension type is added and a older version of the library is used.
    pub fn try_extension_type(&self) -> Result<ExtensionType, Error> {
        self.data[0].try_into()
    }

    /// Returns the extension type.
    ///
    /// This method is similar to `try_extension_type`, but panics if the `u32` value on the
    /// account data cannot be converted to an `ExtensionType`.
    pub fn extension_type(&self) -> ExtensionType {
        self.data[0].try_into().unwrap()
    }

    pub fn set_extension_type(&mut self, extension_type: ExtensionType) {
        self.data[0] = extension_type.into();
    }

    pub fn length(&self) -> u32 {
        self.data[1]
    }

    pub fn set_length(&mut self, length: u32) {
        self.data[1] = length;
    }

    /// Returns the boundary of the extension.
    ///
    /// The boundary is the number of bytes from the start of the extension to the start
    /// of the next extension. This is used specify any padding that may be required to
    /// maintain byte alignment.
    ///
    /// Note that the boundary might be larger than the length of the extension data.
    pub fn boundary(&self) -> u32 {
        self.data[2]
    }

    pub fn set_boundary(&mut self, boundary: u32) {
        self.data[2] = boundary;
    }

    /// Returns the extension data of a given type.
    ///
    /// This function expects a slice of bytes of extension data only and it will return the first
    /// extension of the given type; if the extension type is not found, `None` is returned.
    pub fn get<'a, T: ExtensionData<'a>>(data: &'a [u8]) -> Option<T> {
        let mut cursor = 0;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

            match extension.try_extension_type() {
                Ok(t) if t == T::TYPE => {
                    let start = cursor + Extension::LEN;
                    let end = start + extension.length() as usize;
                    return Some(T::from_bytes(&data[start..end]));
                }
                Ok(ExtensionType::None) => return None,
                _ => cursor = extension.boundary() as usize - Asset::LEN,
            }
        }

        None
    }

    /// Returns a mutable reference to the extension data of a given type.
    ///
    /// This function expects a slice of bytes of extension data only and it will return the first
    /// extension of the given type; if the extension type is not found, `None` is returned.
    pub fn get_mut<'a, T: ExtensionDataMut<'a>>(data: &'a mut [u8]) -> Option<T> {
        let mut cursor = 0;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

            match extension.try_extension_type() {
                Ok(t) if t == T::TYPE => {
                    let start = cursor + Extension::LEN;
                    let end = start + extension.length() as usize;
                    return Some(T::from_bytes_mut(&mut data[start..end]));
                }
                Ok(ExtensionType::None) => return None,
                _ => cursor = extension.boundary() as usize - Asset::LEN,
            }
        }

        None
    }
}

impl Debug for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extension")
            .field("type", &self.extension_type())
            .field("length", &self.length())
            .field("boundary", &self.boundary())
            .finish()
    }
}

/// Default implementation for zero-copy trait.
impl<'a> ZeroCopy<'a, Extension> for Extension {}

/// Trait for extension data.
///
/// The `ExtensionData` defines the data of a particular extension. To implement this trait,
/// a type also needs to implement the `Lifecycle` trait to manage the lifecycle of the extension.
pub trait ExtensionData<'a> {
    const TYPE: ExtensionType;

    fn from_bytes(bytes: &'a [u8]) -> Self;

    fn length(&self) -> usize;
}

pub trait ExtensionDataMut<'a>: Lifecycle {
    const TYPE: ExtensionType;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self;
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExtensionType {
    None,
    Attributes,
    Blob,
    Creators,
    Links,
    Metadata,
    Grouping,
    Royalties,
    Manager,
    Proxy,
}

impl TryFrom<u32> for ExtensionType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ExtensionType::None),
            1 => Ok(ExtensionType::Attributes),
            2 => Ok(ExtensionType::Blob),
            3 => Ok(ExtensionType::Creators),
            4 => Ok(ExtensionType::Links),
            5 => Ok(ExtensionType::Metadata),
            6 => Ok(ExtensionType::Grouping),
            7 => Ok(ExtensionType::Royalties),
            8 => Ok(ExtensionType::Manager),
            9 => Ok(ExtensionType::Proxy),
            _ => Err(Error::InvalidExtensionType(value)),
        }
    }
}

impl From<ExtensionType> for u32 {
    fn from(value: ExtensionType) -> Self {
        match value {
            ExtensionType::None => 0,
            ExtensionType::Attributes => 1,
            ExtensionType::Blob => 2,
            ExtensionType::Creators => 3,
            ExtensionType::Links => 4,
            ExtensionType::Metadata => 5,
            ExtensionType::Grouping => 6,
            ExtensionType::Royalties => 7,
            ExtensionType::Manager => 8,
            ExtensionType::Proxy => 9,
        }
    }
}

/// Trait for building an extension.
///
/// The `ExtensionBuilder` encapsulates the logic for building an extension by allocating the
/// necessary memory and writing the extension data to a buffer. The `data` method can then
/// be used to get retrieve the bytes buffer and the `build` method can be used to create the
/// extension from the buffer.
pub trait ExtensionBuilder<'a, T: ExtensionData<'a>>: Default + Deref {
    /// Builds the extension from the data buffer.
    fn build(&'a self) -> T;

    /// Returns the data buffer.
    fn data(&mut self) -> Vec<u8>;
}

/// Trait to define lifecycle callbacks for an extension.
pub trait Lifecycle {
    /// Validates the data of the extension.
    fn on_create(&mut self) -> Result<(), Error> {
        Ok(())
    }

    /// Validates the data of the extension when it is updated.
    ///
    /// The purpose of this callback is to provide a mechanism to validate and modify the data of an
    /// extension when it is updated.
    fn on_update(&mut self, _other: &mut Self) -> Result<(), Error> {
        Ok(())
    }
}

/// Defines "generic" lifecycle functions for extension types.
///
/// This macro is used to generate helper functions to call `on_create` and `on_update` for
/// each extension type. Note that these functions are only called on types that implement the
/// `Lifecycle` trait.
macro_rules! validate_extension_type {
    ( $( ($member:tt, $member_mut:tt) ),+ $(,)? ) => {
        #[inline(always)]
        pub fn on_create(
            extension_type: ExtensionType,
            data: &mut [u8],
        ) -> Result<(), Error>{
            match extension_type {
                $(
                    ExtensionType::$member => $member_mut::from_bytes_mut(data).on_create(),
                )+
                _ => Ok(()),
            }
        }

        #[inline(always)]
        pub fn on_update(
            extension_type: ExtensionType,
            data: &mut [u8],
            updated: &mut [u8],
        ) -> Result<(), Error>{
            match extension_type {
                $(
                    ExtensionType::$member => $member_mut::from_bytes_mut(data).on_update(
                        &mut $member_mut::from_bytes_mut(updated)
                    ),
                )+
                _ => Ok(()),
            }
        }
    };
}

validate_extension_type!(
    (Attributes, AttributesMut),
    (Blob, BlobMut),
    (Creators, CreatorsMut),
    (Grouping, GroupingMut),
    (Links, LinksMut),
    (Metadata, MetadataMut),
    (Royalties, RoyaltiesMut),
    (Manager, ManagerMut),
    (Proxy, ProxyMut)
);
