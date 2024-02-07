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
mod links;
mod metadata;
mod royalties;

pub use attributes::*;
pub use blob::*;
pub use creators::*;
pub use links::*;
pub use metadata::*;
pub use royalties::*;

use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;

use crate::validation::Validatable;

/// The `Extension` struct is used to store the "header" information for an extension.
///
/// This information is added at the start of each extension data and it is used to determine
/// the type of extension and the length of the extension data.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
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

    pub fn extension_type(&self) -> ExtensionType {
        self.data[0].into()
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
}

/// Default implementation for zero-copy trait.
impl<'a> ZeroCopy<'a, Extension> for Extension {}

/// Trait for extension data.
///
/// The `ExtensionData` defines the data of a particular extension. To implement this trait,
/// a type also
pub trait ExtensionData<'a>: Validatable {
    const TYPE: ExtensionType;

    fn from_bytes(bytes: &'a [u8]) -> Self;

    fn length(&self) -> usize;
}

pub trait ExtensionDataMut<'a> {
    const TYPE: ExtensionType;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self;

    fn length(&self) -> usize;
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum ExtensionType {
    None,
    Attributes,
    Blob,
    Creators,
    Links,
    Metadata,
    Royalties,
}

impl From<u32> for ExtensionType {
    fn from(value: u32) -> Self {
        match value {
            0 => ExtensionType::None,
            1 => ExtensionType::Attributes,
            2 => ExtensionType::Blob,
            3 => ExtensionType::Creators,
            4 => ExtensionType::Links,
            5 => ExtensionType::Metadata,
            6 => ExtensionType::Royalties,
            _ => panic!("invalid extension value: {value}"),
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
            ExtensionType::Royalties => 6,
        }
    }
}

/// Trait for building an extension.
///
/// The `ExtensionBuilder` encapsulates the logic for building an extension by allocating the
/// necessary memory and writing the extension data to a buffer. The `build` method can then
/// be used to get retrieve the data buffer.
pub trait ExtensionBuilder: Default {
    const TYPE: ExtensionType;

    fn build(&mut self) -> Vec<u8>;
}

/// Defines a "generic" validate function.
///
/// This macro is used to generate a helper validate function that can validate any extension type.
macro_rules! validate_extension_type {
    ($($member:tt),+ $(,)?) => {
        pub fn validate(
            extension_type: ExtensionType,
            data: &[u8],
        ) -> Result<(), $crate::validation::ValidationError>{
            match extension_type {
                $(
                    ExtensionType::$member => $member::from_bytes(data).validate(),
                )+
                ExtensionType::None => Ok(()),
            }
        }
    };
}

validate_extension_type!(Attributes, Blob, Creators, Links, Metadata, Royalties);
