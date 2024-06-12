use bytemuck::{Pod, Zeroable};
use podded::pod::{Nullable, PodOption};
use solana_program::pubkey::Pubkey;
use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{error::Error, state::NullablePubkey};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Empty string used for backwards compatibility with metadata extension.
const EMPTY: [u8; 32] = [0u8; 32];

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

    /// An optional delegate authorised to add assets to this group
    pub delegate: &'a PodOption<NullablePubkey>,
}

impl<'a> ExtensionData<'a> for Grouping<'a> {
    const TYPE: ExtensionType = ExtensionType::Grouping;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (size, rest) = bytes.split_at(std::mem::size_of::<u64>());
        let (max_size, delegate) = rest.split_at(std::mem::size_of::<u64>());

        Self {
            size: bytemuck::from_bytes(size),
            max_size: bytemuck::from_bytes(max_size),
            // backwards compatibility for grouping extension: if there are not enough
            // bytes to read the delegate, we assume it is empty
            delegate: bytemuck::from_bytes(if delegate.len() < EMPTY.len() {
                &EMPTY
            } else {
                delegate
            }),
        }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<u64>() + std::mem::size_of::<u64>() + std::mem::size_of::<Pubkey>()
    }
}

impl Debug for Grouping<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Group")
            .field("size", &self.size)
            .field("max_size", &self.max_size.value())
            .field("delegate", &self.delegate.value())
            .finish()
    }
}

pub struct GroupingMut<'a> {
    pub size: &'a mut u64,

    pub max_size: &'a mut PodOption<NullableU64>,

    pub delegate: &'a mut PodOption<NullablePubkey>,
}

impl<'a> ExtensionDataMut<'a> for GroupingMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Grouping;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (size, rest) = bytes.split_at_mut(std::mem::size_of::<u64>());
        let (max_size, delegate) = rest.split_at_mut(std::mem::size_of::<u64>());

        Self {
            size: bytemuck::from_bytes_mut(size),
            max_size: bytemuck::from_bytes_mut(max_size),
            // backwards compatibility for grouping extension: if there are not enough
            // bytes to read the delegate, we assume it is empty
            delegate: bytemuck::from_bytes_mut(if delegate.len() < EMPTY.len() {
                unsafe { (&EMPTY as *const [u8] as *mut [u8]).as_mut().unwrap() }
            } else {
                delegate
            }),
        }
    }
}

impl Lifecycle for GroupingMut<'_> {
    fn on_create(&mut self, _authority: Option<&Pubkey>) -> Result<(), super::Error> {
        if *self.size > 0 {
            Err(Error::InvalidGroupSize)
        } else {
            Ok(())
        }
    }

    fn on_update(&mut self, other: &mut Self, _authority: Option<&Pubkey>) -> Result<(), Error> {
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
pub struct GroupingBuilder(Vec<u8>);

impl Default for GroupingBuilder {
    fn default() -> Self {
        Self(vec![
            0;
            (std::mem::size_of::<u64>() * 2)
                + std::mem::size_of::<Pubkey>()
        ])
    }
}

impl GroupingBuilder {
    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    /// Add a new attribute to the extension.
    pub fn set(&mut self, max_size: Option<u64>, delegate: Option<&Pubkey>) -> &mut Self {
        // setting the data replaces any existing data
        self.0.clear();

        self.0.extend_from_slice(&u64::to_le_bytes(0));
        self.0
            .extend_from_slice(&u64::to_le_bytes(max_size.unwrap_or(0)));

        if let Some(delegate) = delegate {
            self.0.extend_from_slice(delegate.as_ref());
        } else {
            self.0.extend_from_slice(Pubkey::default().as_ref());
        }

        self
    }
}

impl<'a> ExtensionBuilder<'a, Grouping<'a>> for GroupingBuilder {
    fn build(&'a self) -> Grouping<'a> {
        Grouping::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for GroupingBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use solana_program::sysvar;
    use std::ops::Deref;

    use crate::extensions::{ExtensionBuilder, GroupingBuilder};

    #[test]
    fn test_set_max_size() {
        // max_size set
        let mut builder = GroupingBuilder::default();
        builder.set(Some(10), None);
        let grouping = builder.build();

        assert_eq!(*grouping.size, 0);
        assert!(grouping.max_size.value().is_some());

        let max_size = grouping.max_size.value().unwrap();
        assert_eq!(**max_size, 10);

        // "default" max size

        let builder = GroupingBuilder::default();
        let grouping = builder.build();

        assert_eq!(*grouping.size, 0);
        assert!(grouping.max_size.value().is_none());
        assert!(grouping.delegate.value().is_none());
    }

    #[test]
    fn test_set_delegate() {
        // set delegate to a pubkey
        let mut builder = GroupingBuilder::default();
        builder.set(None, Some(&sysvar::ID));
        let grouping = builder.build();

        assert!(grouping.delegate.value().is_some());

        if let Some(delegate) = grouping.delegate.value() {
            assert_eq!(delegate.deref(), &sysvar::ID);
        }

        // set delegate to None
        builder.set(None, None);
        let grouping = builder.build();

        assert!(grouping.delegate.value().is_none());
    }
}
