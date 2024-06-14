use nifty_asset_types::{
    podded::{pod::PodBool, ZeroCopy},
    state::{Asset, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, Handover},
    require,
};

/// Handover an asset to a new authority.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` authority
///   2. `[signer]` new_authority
pub fn process_handover(program_id: &Pubkey, ctx: Context<Handover>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.authority.is_signer(),
        ProgramError::MissingRequiredSignature,
        "authority"
    );

    require!(
        ctx.accounts.new_authority.is_signer(),
        ProgramError::MissingRequiredSignature,
        "new_authority"
    );

    require!(
        ctx.accounts.asset.owner() == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let mut data = ctx.accounts.asset.try_borrow_mut_data()?;

    require!(
        data.len() >= Asset::LEN && data[0] == u8::from(Discriminator::Asset),
        AssetError::Uninitialized,
        "asset"
    );

    let asset = Asset::load_mut(&mut data);

    require!(
        asset.authority == *ctx.accounts.authority.key(),
        AssetError::InvalidAuthority,
        "authority"
    );

    require!(
        <PodBool as Into<bool>>::into(asset.mutable),
        AssetError::ImmutableAsset,
        "asset"
    );

    asset.authority = *ctx.accounts.new_authority.key();

    Ok(())
}
