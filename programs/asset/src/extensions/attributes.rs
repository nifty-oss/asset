use bytemuck::{Pod, Zeroable};
use podded::types::PodStr;

use super::{ExtensionData, ExtensionDataMut, ExtensionType};

/// Extension to add attributes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Attributes<'a> {
    pub traits: &'a [Trait],
}

impl<'a> Attributes<'a> {
    pub fn length_for_capacity(capacity: usize) -> usize {
        capacity * std::mem::size_of::<Trait>()
    }
}

impl<'a> ExtensionData<'a> for Attributes<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let traits = bytemuck::cast_slice(bytes);
        Self { traits }
    }

    fn length(&self) -> usize {
        std::mem::size_of_val(self.traits)
    }
}

/// Extension to add attributes.
#[repr(C)]
pub struct AttributesMut<'a> {
    pub traits: &'a mut [Trait],
}

impl<'a> ExtensionDataMut<'a> for AttributesMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let traits = bytemuck::cast_slice_mut(bytes);
        Self { traits }
    }

    fn length(&self) -> usize {
        std::mem::size_of_val(self.traits)
    }
}

/// Creator information.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Trait {
    /// Name of the trait.
    pub trait_type: PodStr<16>,

    /// Value of the trait.
    pub value: PodStr<16>,
}
