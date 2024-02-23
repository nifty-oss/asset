use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;
use solana_program::pubkey::Pubkey;
use std::{fmt::Debug, ops::Deref};

use crate::error::Error;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Maximum total share of royalties.
const TOTAL_SHARE: u8 = 100;

/// Extension to add a list of creators.
///
/// This extension supports a variable number of creators. The only restriction is
/// that the total share of royalties must be `100`.
pub struct Creators<'a> {
    /// List of creators.
    pub creators: &'a [Creator],
}

impl Creators<'_> {
    /// Returns the creator with the given address.
    pub fn get(&self, address: &Pubkey) -> Option<&Creator> {
        self.creators
            .iter()
            .find(|creator| &creator.address == address)
    }
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

/// Mutable version of the `Creators` extension.
pub struct CreatorsMut<'a> {
    pub creators: &'a mut [Creator],
}

impl CreatorsMut<'_> {
    /// Returns the creator with the given address.
    pub fn get(&mut self, address: &Pubkey) -> Option<&mut Creator> {
        self.creators
            .iter_mut()
            .find(|&&mut creator| &creator.address == address)
    }
}

impl<'a> ExtensionDataMut<'a> for CreatorsMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Creators;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let creators = bytemuck::cast_slice_mut(bytes);
        Self { creators }
    }
}

impl Lifecycle for CreatorsMut<'_> {
    /// Validates the creators' share added up to `100`.
    fn on_create(&mut self) -> Result<(), Error> {
        let mut total = 0;

        self.creators.iter_mut().for_each(|creator| {
            // make sure all creators are unverified
            creator.set_verified(false);
            total += creator.share();
        });

        if total != TOTAL_SHARE {
            Err(Error::InvalidCreatorsTotalShare(TOTAL_SHARE, total))
        } else {
            Ok(())
        }
    }

    fn on_update(&mut self, other: &mut Self) -> Result<(), Error> {
        let mut total = 0;
        other.creators.iter_mut().for_each(|creator| {
            if let Some(original) = self.get(&creator.address) {
                // creators maintain their verified status
                creator.set_verified(original.verified());
            } else {
                // creators are always initialized as unverified
                creator.set_verified(false);
            }
            total += creator.share();
        });

        if total != 100 {
            return Err(Error::InvalidCreatorsTotalShare(TOTAL_SHARE, total));
        }

        for creator in self.creators.iter() {
            // if the creator is verified, it must be present in
            // the other list
            if creator.verified() {
                if let Some(updated) = other.get(&creator.address) {
                    // sanity check: the creator must have already been verified
                    // on the above closure
                    if !updated.verified() {
                        return Err(Error::CannotUnverifyCreator);
                    }
                } else {
                    // cannot remove a verified creator
                    return Err(Error::CannotRemoveVerifiedCreator);
                }
            }
        }

        Ok(())
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

impl Debug for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Creator")
            .field("address", &self.address)
            .field("verified", &self.verified())
            .field("share", &self.share())
            .finish()
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
