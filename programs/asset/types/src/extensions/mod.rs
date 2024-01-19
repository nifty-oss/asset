//! Extensions are used to add additional data to an asset.
//!
//! It is possible to attach additional data to an asset using extensions. The `Extension`
//! struct provides the "header" information for the extension, and stores the type of the
//! extension, the length of the extension data, and the boundary of the extension on the
//! account data buffer.
//!
//! The type and length are determined by the extension itself. The boundary is used internally
//! to make sure that each extensions data is aligned to a 8-byte boundary. This is required to
//! support extensions that store their data using bytemuck's `Pod` trait.

mod attributes;
mod blob;
mod creators;
mod links;

pub use attributes::*;
pub use blob::*;
pub use creators::*;
pub use links::*;

use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;

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

impl<'a> ZeroCopy<'a, Extension> for Extension {}

pub trait ExtensionData<'a> {
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
}

impl From<u32> for ExtensionType {
    fn from(value: u32) -> Self {
        match value {
            0 => ExtensionType::None,
            1 => ExtensionType::Attributes,
            2 => ExtensionType::Blob,
            3 => ExtensionType::Creators,
            4 => ExtensionType::Links,
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
        }
    }
}

/// Trait for building an extension.
///
/// The `ExtensionBuilder` encapsulates the logic for building an extension by allocating the
/// necessary memory and writing the extension data to the buffer. The `build` method can then
/// be used to get the extension data.
pub trait ExtensionBuilder: Default {
    const TYPE: ExtensionType;

    fn build(&mut self) -> Vec<u8>;
}
