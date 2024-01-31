use nifty_asset_types::state::{Asset, Delegate, Discriminator};
use podded::{pod::PodOption, ZeroCopy};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, RevokeAccounts},
    require,
};

pub fn process_revoke(program_id: &Pubkey, ctx: Context<RevokeAccounts>) -> ProgramResult {
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
        data[0] == Discriminator::Asset.into(),
        ProgramError::UninitializedAccount,
        "asset"
    );

    let asset = Asset::load_mut(&mut data);

    let is_holder = asset.holder == *ctx.accounts.signer.key;

    let is_delegate = asset
        .delegate
        .value()
        .map(|delegate| *delegate.address == *ctx.accounts.signer.key);

    // we only revoke a delegate if the signer is the holder or the current delegate
    if is_holder || (is_delegate == Some(true)) {
        asset.delegate = PodOption::new(Delegate::default());
    } else {
        return err!(AssetError::InvalidAuthority, "not a holder or delegate");
    }

    Ok(())
}
