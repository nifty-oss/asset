use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;
use solana_program::pubkey::Pubkey;
use std::ops::Deref;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType};
use crate::validation::{Validatable, ValidationError};

/// Extension to add a list of creators.
///
/// This extension supports a variable number of creators. The only restriction is
/// that the total share of royalties must be 100.
pub struct Creators<'a> {
    /// List of creators.
    pub creators: &'a [Creator],
}

impl<'a> ExtensionData<'a> for Creators<'a> {
    const TYPE: ExtensionType = ExtensionType::Creators;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let creators = bytemuck::cast_slice(bytes);
        Self { creators }
    }

    fn length(&self) -> usize {
        std::mem::size_of_val(self.creators)
    }
}

/// Validatable implementation for `Creators`.
///
/// The total share of royalties must be 100.
impl Validatable for Creators<'_> {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut total = 0;

        for creator in self.creators {
            total += creator.share();
        }

        if total != 100 {
            Err(ValidationError::InvalidShareTotal)
        } else {
            Ok(())
        }
    }
}

/// Mutable version of the `Creators` extension.
pub struct CreatorsMut<'a> {
    pub creators: &'a mut [Creator],
}

impl<'a> ExtensionDataMut<'a> for CreatorsMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Creators;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let creators = bytemuck::cast_slice_mut(bytes);
        Self { creators }
    }
}

/// Creator information.
#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Creator {
    /// Pubkey address.
    pub address: Pubkey,
    /// Additional information.
    ///   0. verified flag
    ///   1. share of royalties
    ///   2-7. unused
    pub data: [u8; 8],
}

impl Creator {
    pub fn verified(&self) -> bool {
        self.data[0] == 1
    }

    pub fn set_verified(&mut self, verified: bool) {
        self.data[0] = verified as u8;
    }

    pub fn share(&self) -> u8 {
        self.data[1]
    }

    pub fn set_share(&mut self, share: u8) {
        self.data[1] = share;
    }
}

impl ZeroCopy<'_, Creator> for Creator {}

/// Builder for a `Creators` extension.
#[derive(Default)]
pub struct CreatorsBuilder {
    /// The current number of creators.
    ///
    /// There is a maximum of 5 creators.
    count: u8,

    /// The extension data.
    data: Vec<u8>,
}

impl CreatorsBuilder {
    /// Add a new creator to the extension.
    pub fn add(&mut self, addresss: &Pubkey, verified: bool, share: u8) {
        // extends the data buffer
        self.data
            .append(&mut vec![0u8; std::mem::size_of::<Creator>()]);
        let offset = self.count as usize * std::mem::size_of::<Creator>();

        let creator = Creator::load_mut(&mut self.data[offset..]);
        creator.address = *addresss;
        creator.data = [verified as u8, share, 0, 0, 0, 0, 0, 0];

        self.count += 1;
    }
}

impl ExtensionBuilder for CreatorsBuilder {
    const TYPE: ExtensionType = ExtensionType::Creators;

    fn build(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.data)
    }
}

impl Deref for CreatorsBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{CreatorsBuilder, ExtensionData};
    use solana_program::pubkey;

    use super::Creators;

    #[test]
    fn test_add() {
        let mut builder = CreatorsBuilder::default();
        builder.add(
            &pubkey!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73"),
            true,
            100,
        );
        let list = Creators::from_bytes(&builder);
        assert_eq!(list.creators.len(), 1);
        assert_eq!(
            list.creators[0].address,
            pubkey!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73")
        );
        assert!(list.creators[0].verified());
        assert_eq!(list.creators[0].share(), 100);
    }
}
