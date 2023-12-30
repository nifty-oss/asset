use bytemuck::{Pod, Zeroable};
use podded::{
    types::{PodBool, PodStr},
    ZeroCopy,
};
use shank::ShankType;
use solana_program::{
    account_info::AccountInfo, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey,
    rent::Rent, system_instruction, sysvar::Sysvar,
};

use super::{Delegate, Discriminator};
use crate::extensions::{Extension, ExtensionData, ExtensionType};

/// Maximum length of a name.
pub const MAX_NAME_LENGTH: usize = 32;

/// Maximum length of a symbol.
pub const MAX_SYMBOL_LENGTH: usize = 10;

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct Asset {
    /// Account discriminator.
    pub discriminator: Discriminator,

    /// State of the asset.
    pub state: State,

    /// The PDA derivation bump.
    pub bump: u8,

    /// Indicates whether the asset is mutable.
    pub mutable: PodBool,

    /// Current holder of the asset.
    pub holder: Pubkey,

    /// Group of the asset.
    ///
    /// This is a reference to the asset that represents the group. When
    /// the asset is not part of a group, the group is zero'd out.
    pub group: Pubkey,

    /// Authority of the asset.
    ///
    /// The authority is the account that can update the metadata of the
    /// asset. This is typically the creator of the asset.
    pub authority: Pubkey,

    /// Delegate of the asset.
    ///
    /// The delegate is the account that can control the asset on behalf of
    /// the holder.
    pub delegate: Delegate,

    /// Name of the asset.
    pub name: PodStr<MAX_NAME_LENGTH>,

    /// Name of the asset.
    pub symbol: PodStr<MAX_SYMBOL_LENGTH>,
}

impl Asset {
    /// Length of the account data.
    // Note: +1 to have 8 byte alignment since extension data might have
    // a different alignment.
    pub const LEN: usize = std::mem::size_of::<Asset>() + 1;

    pub const SEED: &'static str = "asset";

    /// Allocates space to store an extension in a given account.
    ///
    /// This is used to allocate space for the extension data. The function
    /// return the offset represeting the start of the allocated space.
    pub fn allocate<'a, 'b>(
        account: &'b AccountInfo<'a>,
        payer: &'b AccountInfo<'a>,
        system_program: &'b AccountInfo<'a>,
        extension: Extension,
        size: usize,
    ) -> Result<usize, ProgramError> {
        let offset = account.data_len();
        let extended = offset + Extension::LEN + size;
        let required_rent = Rent::get()?
            .minimum_balance(extended)
            .saturating_sub(account.lamports());

        msg!("Funding {} lamports for account realloc", required_rent);

        invoke(
            &system_instruction::transfer(payer.key, account.key, required_rent),
            &[payer.clone(), account.clone(), system_program.clone()],
        )?;

        account.realloc(extended, false)?;

        let data = &mut (*account.data).borrow_mut();
        let location = Extension::load_mut(&mut data[offset..]);
        *location = extension;

        Ok(Extension::LEN + offset)
    }

    pub fn get<'a, T: ExtensionData<'a>>(data: &'a [u8]) -> Option<T> {
        let mut cursor = Asset::LEN;

        while cursor < data.len() {
            let header = Extension::load(&data[cursor..]);
            cursor = cursor.saturating_add(Extension::LEN);

            if header.extension_type() == T::TYPE {
                return Some(T::from_bytes(&data[cursor..]));
            }

            cursor = cursor.saturating_add(header.length() as usize);
        }

        None
    }

    pub fn contains(extension_type: ExtensionType, data: &[u8]) -> bool {
        let mut cursor = Asset::LEN;

        while cursor < data.len() {
            let header = Extension::load(&data[cursor..]);
            cursor = cursor.saturating_add(Extension::LEN);

            if header.extension_type() == extension_type {
                return true;
            }

            cursor = cursor.saturating_add(header.length() as usize);
        }

        false
    }

    pub fn get_extensions(data: &[u8]) -> Vec<ExtensionType> {
        let mut cursor = Asset::LEN;
        let mut extensions = Vec::new();

        while cursor < data.len() {
            let extension = Extension::load(&data[cursor..]);
            extensions.push(extension.extension_type());
            cursor = cursor
                .saturating_add(Extension::LEN)
                .saturating_add(extension.length() as usize);
        }

        extensions
    }

    pub fn first_extension(data: &[u8]) -> Option<(&Extension, usize)> {
        if Asset::LEN < data.len() {
            return Some((Extension::load(&data[Asset::LEN..]), Asset::LEN));
        }

        None
    }

    pub fn last_extension(data: &[u8]) -> Option<(&Extension, usize)> {
        let mut cursor = Asset::LEN;
        let mut last = None;

        while cursor < data.len() {
            let extension = Extension::load(&data[cursor..]);
            last = Some((extension, cursor));
            cursor = cursor
                .saturating_add(Extension::LEN)
                .saturating_add(extension.length() as usize);
        }

        last
    }
}

impl<'a> ZeroCopy<'a, Asset> for Asset {}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum State {
    #[default]
    Unlocked,
    Locked,
}

unsafe impl Pod for State {}

unsafe impl Zeroable for State {}
