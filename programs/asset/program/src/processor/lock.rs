use nifty_asset_types::{
    extensions::{Extension, Manager},
    podded::ZeroCopy,
    state::{Asset, DelegateRole, Discriminator, State},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, LockAccounts},
    require,
    utils::assert_delegate,
};

/// Locks an asset.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` signer
pub fn process_lock(program_id: &Pubkey, ctx: Context<LockAccounts>) -> ProgramResult {
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

    // locks the asset

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
            ctx.accounts.signer.key,
            DelegateRole::Lock,
        )?;
    }
    // otherwise, if the signer is not the owner, the signer must be the
    // manager delegate
    else if asset.owner != *ctx.accounts.signer.key {
        assert_delegate(&[manager], ctx.accounts.signer.key, DelegateRole::Lock)?;
    }

    asset.state = State::Locked;

    Ok(())
}
