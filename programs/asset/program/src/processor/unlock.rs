use nifty_asset_types::{
    extensions::{Extension, Manager},
    podded::ZeroCopy,
    state::{Asset, DelegateRole, Discriminator, State},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, UnlockAccounts},
    require,
    utils::assert_delegate,
};

pub fn process_unlock(program_id: &Pubkey, ctx: Context<UnlockAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.authority.is_signer,
        ProgramError::MissingRequiredSignature,
        "delegate"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data[0] == Discriminator::Asset.into(),
        ProgramError::UninitializedAccount,
        "asset"
    );

    // unlocks the asset

    let (asset, extensions) = data.split_at_mut(Asset::LEN);
    let asset = Asset::load_mut(asset);

    if asset.delegate.value().is_some() {
        assert_delegate(
            &[
                asset.delegate.value(),
                Extension::get::<Manager>(extensions).map(|s| s.delegate),
            ],
            ctx.accounts.authority.key,
            DelegateRole::Lock,
        )?;
    } else if asset.owner != *ctx.accounts.authority.key {
        return err!(AssetError::InvalidAuthority);
    }

    asset.state = State::Unlocked;

    Ok(())
}
