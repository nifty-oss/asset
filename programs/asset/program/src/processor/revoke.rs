use nifty_asset_types::{
    podded::{pod::PodOption, ZeroCopy},
    state::{Asset, Delegate, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{Context, RevokeAccounts},
        DelegateInput,
    },
    require,
};

pub fn process_revoke(
    program_id: &Pubkey,
    ctx: Context<RevokeAccounts>,
    args: DelegateInput,
) -> ProgramResult {
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
        data[0] == Discriminator::Asset as u8,
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
    if !(is_holder || is_delegate == Some(true)) {
        return err!(AssetError::InvalidAuthority, "not a holder or delegate");
    }

    // If the All role is passed in we completely revoke the delegate

    match args {
        DelegateInput::All => {
            asset.delegate = PodOption::new(Delegate::default());
            return Ok(());
        }
        DelegateInput::Some { roles } => {
            // otherwise we only disable the roles passed in
            if let Some(delegate) = asset.delegate.value_mut() {
                roles.iter().for_each(|role| delegate.disable(*role));

                if !delegate.has_active_roles() {
                    // and if the delegate has no active roles, then we completely revoke it
                    asset.delegate = PodOption::new(Delegate::default());
                }
            }
        }
    }

    Ok(())
}
