use nifty_asset_types::{
    podded::{pod::PodBool, ZeroCopy},
    state::{Asset, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{Context, Resize},
        SizeInput,
    },
    processor::resize,
    require,
};

/// Resizes an asset account.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` authority
///   2. `[signer, writable]` payer
///   3. `[optional]` system_program
#[inline(always)]
pub fn process_resize(program_id: &Pubkey, ctx: Context<Resize>, args: SizeInput) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.authority.is_signer(),
        ProgramError::MissingRequiredSignature,
        "authority"
    );

    require!(
        ctx.accounts.asset.owner() == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let account_data = ctx.accounts.asset.try_borrow_data()?;

    require!(
        account_data.len() >= Asset::LEN && account_data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "asset"
    );

    let asset = Asset::load(&account_data);

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

    // resizes the asset account

    let size = match args {
        SizeInput::Fit => Asset::last_extension(&account_data)
            .map_or(Asset::LEN, |(extension, _)| extension.boundary() as usize),
        SizeInput::Extend { value } => account_data.len().saturating_add(value as usize),
    };

    drop(account_data);

    resize(
        size,
        ctx.accounts.asset,
        Some(ctx.accounts.payer),
        ctx.accounts.system_program,
    )
}
