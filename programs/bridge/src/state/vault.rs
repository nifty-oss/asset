use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;
use solana_program::pubkey::Pubkey;

use super::{Discriminator, State};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Vault {
    /// Account discriminator.
    pub discriminator: Discriminator,

    /// State of the vault.
    pub state: State,

    /// Derivation bump seed for the vault.
    pub bump: u8,

    /// Mint address.
    pub mint: Pubkey,

    /// Derivation bump seed for the bridged asset.
    pub asset_bump: u8,
}

impl Vault {
    /// Length of the account data.
    pub const LEN: usize = std::mem::size_of::<Vault>();

    /// Prefix value for the PDA derivation.
    pub const PREFIX: &'static [u8] = b"nifty::bridge::vault";
}

impl ZeroCopy<'_, Vault> for Vault {}
