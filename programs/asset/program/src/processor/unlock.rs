use nifty_asset_types::{
    extensions::{Extension, Manager},
    podded::ZeroCopy,
    state::{Asset, DelegateRole, Discriminator, State},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, Unlock},
    require,
    utils::assert_delegate,
};

/// Unlocks an asset.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` signer
pub fn process_unlock(program_id: &Pubkey, ctx: Context<Unlock>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.signer.is_signer(),
        ProgramError::MissingRequiredSignature,
        "signer"
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

    // unlocks the asset

    let (asset, extensions) = data.split_at_mut(Asset::LEN);
    let asset = Asset::load_mut(asset);
    let manager = Extension::get::<Manager>(extensions).map(|s| s.delegate);

    // Validate whether signer is the owner or a lock delegate.
    //
    // if the asset has a delegate, the signer must be the delegate or the manager
    // delegate (if there is one)
    if asset.delegate.value().is_some() {
        assert_delegate(
            &[asset.delegate.value(), manager],
            ctx.accounts.signer.key(),
            DelegateRole::Lock,
        )?;
    }
    // otherwise, if the signer is not the owner, the signer must be the
    // manager delegate
    else if asset.owner != *ctx.accounts.signer.key() {
        assert_delegate(&[manager], ctx.accounts.signer.key(), DelegateRole::Lock)?;
    }

    asset.state = State::Unlocked;

    Ok(())
}
