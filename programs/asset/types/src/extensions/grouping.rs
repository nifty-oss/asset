use bytemuck::{Pod, Zeroable};
use podded::pod::{Nullable, PodOption};
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType};
use crate::validation::Validatable;

/// Extension to define a group of assets.
///
/// Assets that are intented to be use as group "markers" must have this extension
/// attached to them.
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

impl Validatable for Grouping<'_> {}

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
