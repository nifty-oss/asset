use nifty_asset_types::{
    extensions::{Extension, Manager},
    podded::ZeroCopy,
    state::{Asset, DelegateRole, Discriminator, State},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, UnlockAccounts},
    require,
    utils::assert_delegate,
};

/// Unlocks an asset.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` signer
pub fn process_unlock(program_id: &Pubkey, ctx: Context<UnlockAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.signer.is_signer,
        ProgramError::MissingRequiredSignature,
        "signer"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data.len() >= Asset::LEN && data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "asset"
    );

    // unlocks the asset

    let (asset, extensions) = data.split_at_mut(Asset::LEN);
    let asset = Asset::load_mut(asset);

    // Validate the signer is the owner or the lock delegate.
    //
    // it can be the manager delegate; or if the asset has a delegate, the signer must
    // be the delegate; otherwise, the signer must be the owner
    let is_allowed = (asset.delegate.value().is_none() && asset.owner == *ctx.accounts.signer.key)
        || assert_delegate(
            &[
                asset.delegate.value(),
                Extension::get::<Manager>(extensions).map(|s| s.delegate),
            ],
            ctx.accounts.signer.key,
            DelegateRole::Lock,
        )
        .is_ok();

    require!(is_allowed, AssetError::InvalidAssetOwner, "signer");

    asset.state = State::Unlocked;

    Ok(())
}
