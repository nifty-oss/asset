use bytemuck::{Pod, Zeroable};
use podded::pod::{Nullable, PodOption};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::error::Error;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to define a group of assets.
///
/// Assets that are intented to be use as group "markers" must have this extension
/// attached to them.
///
/// The `size` of the group is updated every time an asset is added or removed from the group.
/// Additionally, the `size` is decreased when an asset is burned.
pub struct Grouping<'a> {
    /// The number of assets in the group.
    pub size: &'a u64,

    /// The maximum number of assets that can be in the group.
    ///
    /// When the group is unlimited, this value is `0`.
    pub max_size: &'a PodOption<NullableU64>,
}

impl<'a> ExtensionData<'a> for Grouping<'a> {
    const TYPE: ExtensionType = ExtensionType::Grouping;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (size, max_size) = bytes.split_at(std::mem::size_of::<u64>());
        Self {
            size: bytemuck::from_bytes(size),
            max_size: bytemuck::from_bytes(max_size),
        }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<u64>() + std::mem::size_of::<u64>()
    }
}

impl Debug for Grouping<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("size", &self.size)
            .field("max_size", &self.max_size.value())
            .finish()
    }
}

pub struct GroupingMut<'a> {
    pub size: &'a mut u64,

    pub max_size: &'a mut PodOption<NullableU64>,
}

impl<'a> ExtensionDataMut<'a> for GroupingMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Grouping;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (size, max_size) = bytes.split_at_mut(std::mem::size_of::<u64>());
        Self {
            size: bytemuck::from_bytes_mut(size),
            max_size: bytemuck::from_bytes_mut(max_size),
        }
    }
}

impl Lifecycle for GroupingMut<'_> {
    fn on_create(&mut self) -> Result<(), super::Error> {
        if *self.size > 0 {
            Err(Error::InvalidGroupSize)
        } else {
            Ok(())
        }
    }

    fn on_update(&mut self, other: &mut Self) -> Result<(), Error> {
        // size cannot be updated
        *other.size = *self.size;

        if let Some(max_size) = other.max_size.value() {
            // it cannot update the max size to be lower than the current size
            if **max_size < *other.size {
                return Err(Error::InvalidMaximumGroupSize(*other.size, **max_size));
            }
        }

        Ok(())
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct NullableU64(u64);

impl NullableU64 {
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

impl Deref for NullableU64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NullableU64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Nullable for NullableU64 {
    fn is_some(&self) -> bool {
        self.0 != 0
    }

    fn is_none(&self) -> bool {
        self.0 == 0
    }
}

/// Builder for a `Group` extension.
pub struct GroupBuilder(Vec<u8>);

impl Default for GroupBuilder {
    fn default() -> Self {
        Self(vec![0; std::mem::size_of::<u64>() * 2])
    }
}

impl GroupBuilder {
    /// Add a new attribute to the extension.
    pub fn set_max_size(&mut self, max_size: u64) {
        // setting the data replaces any existing data
        self.0.clear();

        self.0.extend_from_slice(&u64::to_le_bytes(0));
        self.0.extend_from_slice(&u64::to_le_bytes(max_size));
    }
}

impl ExtensionBuilder for GroupBuilder {
    const TYPE: ExtensionType = ExtensionType::Grouping;

    fn build(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for GroupBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{ExtensionData, GroupBuilder, Grouping};

    #[test]
    fn test_set() {
        // max_size set
        let mut builder = GroupBuilder::default();
        builder.set_max_size(10);
        let grouping = Grouping::from_bytes(&builder);

        assert_eq!(*grouping.size, 0);
        assert!(grouping.max_size.value().is_some());

        let max_size = grouping.max_size.value().unwrap();
        assert_eq!(**max_size, 10);

        // "default" max size

        let builder = GroupBuilder::default();
        let grouping = Grouping::from_bytes(&builder);

        assert_eq!(*grouping.size, 0);
        assert!(grouping.max_size.value().is_none());
    }
}
