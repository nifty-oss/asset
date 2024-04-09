//! The `Asset` account consists of a fixed length (header) section followed by an optional
//! variable length section (extensions). The header section contains the basic information
//! of an asset, such as the state, standard, name, and owner. The extensions section
//! contains additional data that can be attached to the asset.
//!
//! Extensions can either represent additional on-chain data or used to include pointers to
//! external data. For example, an asset can include its image on-chain as an extension or
//! a pointer to an external image.
//!
//! # Account layout
//!
//! |  *fixed*  |                            *optional*                             |
//! |-----------|-------------------------------------------------------------------|
//! | Asset     | Extension 1 + Extension Data | ... | Extension n + Extension Data |
//! |-----------|------------------------------|-----|------------------------------|
//! | 168 bytes | 16 bytes    + variable bytes | ... | 16 bytes    + variable bytes |

use bytemuck::{Pod, Zeroable};
use podded::{
    pod::{PodBool, PodOption, PodStr},
    ZeroCopy,
};
use solana_program::pubkey::Pubkey;

use super::{Delegate, Discriminator, NullablePubkey, Standard, State};
use crate::extensions::{Extension, ExtensionData, ExtensionDataMut, ExtensionType};

/// Maximum length of a name.
pub const MAX_NAME_LENGTH: usize = 35;

/// `Asset` account (header) information.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Asset {
    /// Account discriminator.
    pub discriminator: Discriminator,

    /// State of the asset.
    pub state: State,

    /// Standard of the asset.
    pub standard: Standard,

    /// Indicates whether the asset is mutable.
    pub mutable: PodBool,

    /// Current owner (holder) of the asset.
    pub owner: Pubkey,

    /// Group of the asset.
    ///
    /// This is a reference to the asset that represents the group. When
    /// the asset is not part of a group, the group is represented by
    /// `Pubkey::default()`.
    pub group: PodOption<NullablePubkey>,

    /// Authority of the asset.
    ///
    /// The authority is the account that can update the metadata of the
    /// asset. This is typically the creator of the asset.
    pub authority: Pubkey,

    /// Delegate of the asset.
    ///
    /// The delegate is the account that can control the asset on behalf of
    /// the owner.
    pub delegate: PodOption<Delegate>,

    /// Name of the asset.
    pub name: PodStr<MAX_NAME_LENGTH>,
}

impl Asset {
    /// Length of the account data.
    pub const LEN: usize = std::mem::size_of::<Asset>();

    /// Indicates whether the account contains an extension of a given type.
    pub fn contains(extension_type: ExtensionType, data: &[u8]) -> bool {
        let mut cursor = Asset::LEN;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

            match extension.try_extension_type() {
                Ok(t) if t == extension_type => return true,
                Ok(ExtensionType::None) => return false,
                _ => cursor = extension.boundary() as usize,
            }
        }

        false
    }

    /// Returns the extension data of a given type.
    ///
    /// This function will return the first extension of the given type. If the
    /// extension is not found, `None` is returned.
    pub fn get<'a, T: ExtensionData<'a>>(data: &'a [u8]) -> Option<T> {
        let mut cursor = Asset::LEN;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

            match extension.try_extension_type() {
                Ok(t) if t == T::TYPE => {
                    let start = cursor + Extension::LEN;
                    let end = start + extension.length() as usize;
                    return Some(T::from_bytes(&data[start..end]));
                }
                Ok(ExtensionType::None) => return None,
                _ => cursor = extension.boundary() as usize,
            }
        }

        None
    }

    /// Returns a mutable reference to the extension data of a given type.
    ///
    /// This function will return the first extension of the given type. If the
    /// extension is not found, `None` is returned.
    pub fn get_mut<'a, T: ExtensionDataMut<'a>>(data: &'a mut [u8]) -> Option<T> {
        let mut cursor = Asset::LEN;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

            match extension.try_extension_type() {
                Ok(t) if t == T::TYPE => {
                    let start = cursor + Extension::LEN;
                    let end = start + extension.length() as usize;
                    return Some(T::from_bytes_mut(&mut data[start..end]));
                }
                Ok(ExtensionType::None) => return None,
                _ => cursor = extension.boundary() as usize,
            }
        }

        None
    }

    /// Returns the extensions on the account data.
    ///
    /// This function will return a list of `ExtensionType`s that are present
    /// on the account data.
    ///
    /// The account data might have extensions that are not recognized if an older
    /// version of the library is used. In this case, this method will only return
    /// the list of recognized extensions.
    pub fn get_extensions(data: &[u8]) -> Vec<ExtensionType> {
        let mut cursor = Asset::LEN;
        let mut extensions = Vec::new();

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

            if let Ok(t) = extension.try_extension_type() {
                if matches!(t, ExtensionType::None) {
                    return extensions;
                }
                extensions.push(t);
            }

            cursor = extension.boundary() as usize;
        }

        extensions
    }

    /// Returns the first extension of the account.
    ///
    /// This function will return a tuple containing the extension type and the
    /// offset of the extension data. If the account does not contain any extension,
    /// `None` is returned.
    pub fn first_extension(data: &[u8]) -> Option<(&Extension, usize)> {
        if (Asset::LEN + Extension::LEN) <= data.len() {
            return Some((
                Extension::load(&data[Asset::LEN..]),
                Asset::LEN + Extension::LEN,
            ));
        }

        None
    }

    /// Returns the last extension of the account.
    ///
    /// This function will return a tuple containing the extension type and the
    /// offset of the extension data. If the account does not contain any extension,
    /// `None` is returned.
    pub fn last_extension(data: &[u8]) -> Option<(&Extension, usize)> {
        let mut cursor = Asset::LEN;
        let mut last = None;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..]);

            if let Ok(ExtensionType::None) = extension.try_extension_type() {
                return last;
            }

            last = Some((extension, cursor + Extension::LEN));
            cursor = extension.boundary() as usize;
        }

        last
    }

    /// Returns the extension given its type.
    ///
    /// This function will return a tuple containing the extension type and the
    /// slice of the extension data. If the account does not contain any extension,
    /// `None` is returned.
    pub fn get_extension(
        extension_type: ExtensionType,
        data: &[u8],
    ) -> Option<(&Extension, usize)> {
        let mut cursor = Asset::LEN;

        while (cursor + Extension::LEN) <= data.len() {
            let extension = Extension::load(&data[cursor..]);

            match extension.try_extension_type() {
                Ok(t) if t == extension_type => return Some((extension, cursor + Extension::LEN)),
                Ok(ExtensionType::None) => return None,
                _ => cursor = extension.boundary() as usize,
            }
        }

        None
    }
}

/// Default implementation for zero-copy trait.
impl<'a> ZeroCopy<'a, Asset> for Asset {}
