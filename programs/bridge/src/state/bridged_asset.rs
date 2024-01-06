use solana_program::pubkey::{Pubkey, PubkeyError};

/// Prefix value for the PDA derivation of bridged assets.
pub const BRIDGED_ASSET_PREFIX: &[u8] = b"nifty::bridge::asset";

pub fn create_pda(mint: Pubkey, bump: u8) -> Result<Pubkey, PubkeyError> {
    solana_program::pubkey::Pubkey::create_program_address(
        &[BRIDGED_ASSET_PREFIX, mint.as_ref(), &[bump]],
        &crate::ID,
    )
}

pub fn find_pda(mint: &Pubkey) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[BRIDGED_ASSET_PREFIX, mint.as_ref()],
        &crate::ID,
    )
}
