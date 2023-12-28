mod attributes;
mod creators;
mod image;

pub use attributes::*;
pub use creators::*;
pub use image::*;

use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct Header {
    /// Data section.
    ///   0. type
    ///   1. length
    data: [u32; 2],
}

impl Header {
    pub const LEN: usize = std::mem::size_of::<Self>();

    pub fn extension_type(&self) -> ExtensionType {
        self.data[0].into()
    }

    pub fn set_extension_type(&mut self, value: ExtensionType) {
        self.data[0] = value.into();
    }

    pub fn length(&self) -> usize {
        self.data[1] as usize
    }

    pub fn set_length(&mut self, value: usize) {
        self.data[1] = value as u32;
    }
}

impl<'a> ZeroCopy<'a, Header> for Header {}

pub trait Extension<'a> {
    const TYPE: ExtensionType;

    fn from_bytes(bytes: &'a [u8]) -> Self;

    fn length(&self) -> usize;
}

pub trait ExtensionMut<'a> {
    const TYPE: ExtensionType;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self;

    fn length(&self) -> usize;
}

#[derive(Clone, Copy, BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum ExtensionType {
    None,
    Attributes,
    Creators,
    Image,
}

impl From<u32> for ExtensionType {
    fn from(value: u32) -> Self {
        match value {
            0 => ExtensionType::None,
            1 => ExtensionType::Attributes,
            2 => ExtensionType::Creators,
            3 => ExtensionType::Image,
            _ => panic!("invalid extension value: {value}"),
        }
    }
}

impl From<ExtensionType> for u32 {
    fn from(value: ExtensionType) -> Self {
        match value {
            ExtensionType::None => 0,
            ExtensionType::Attributes => 1,
            ExtensionType::Creators => 2,
            ExtensionType::Image => 3,
        }
    }
}
