use bytemuck::{Pod, Zeroable};
use podded::{pod::PodBool, ZeroCopy};
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
            creator.verified = false.into();
            total += creator.share;
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
                creator.verified = original.verified;
            } else {
                // creators are always initialized as unverified
                creator.verified = false.into();
            }
            total += creator.share;
        });

        if total != 100 {
            return Err(Error::InvalidCreatorsTotalShare(TOTAL_SHARE, total));
        }

        for creator in self.creators.iter() {
            // if the creator is verified, it must be present in
            // the other list
            if creator.verified.into() {
                if let Some(updated) = other.get(&creator.address) {
                    // sanity check: the creator must have already been verified
                    // on the above closure
                    if !<PodBool as Into<bool>>::into(updated.verified) {
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

    /// Indicates whether the creator is verified or not.
    pub verified: PodBool,

    /// Share of royalties.
    pub share: u8,
}

impl Debug for Creator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Creator")
            .field("address", &self.address)
            .field("verified", &<PodBool as Into<bool>>::into(self.verified))
            .field("share", &self.share)
            .finish()
    }
}

impl ZeroCopy<'_, Creator> for Creator {}

/// Builder for a `Creators` extension.
#[derive(Default)]
pub struct CreatorsBuilder {
    /// The current number of creators.
    count: u8,

    /// The extension data.
    data: Vec<u8>,
}

impl CreatorsBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            count: 0,
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        let mut s = Self {
            count: 0,
            data: buffer,
        };
        s.data.clear();
        s
    }

    /// Add a new creator to the extension.
    pub fn add(&mut self, addresss: &Pubkey, verified: bool, share: u8) -> &mut Self {
        // extends the data buffer
        self.data
            .append(&mut vec![0u8; std::mem::size_of::<Creator>()]);
        let offset = self.count as usize * std::mem::size_of::<Creator>();

        let creator = Creator::load_mut(&mut self.data[offset..]);
        creator.address = *addresss;
        creator.verified = verified.into();
        creator.share = share;

        self.count += 1;

        self
    }
}

impl<'a> ExtensionBuilder<'a, Creators<'a>> for CreatorsBuilder {
    fn build(&'a self) -> Creators<'a> {
        Creators::from_bytes(&self.data)
    }

    fn data(&mut self) -> Vec<u8> {
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
    use crate::extensions::{CreatorsBuilder, ExtensionBuilder};
    use podded::pod::PodBool;
    use solana_program::pubkey;

    #[test]
    fn test_add() {
        let mut builder = CreatorsBuilder::default();
        builder.add(
            &pubkey!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73"),
            true,
            100,
        );
        let list = builder.build();

        assert_eq!(list.creators.len(), 1);
        assert_eq!(
            list.creators[0].address,
            pubkey!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73")
        );
        assert!(<PodBool as Into<bool>>::into(list.creators[0].verified));
        assert_eq!(list.creators[0].share, 100);
    }
}
