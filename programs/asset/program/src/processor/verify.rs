use nifty_asset_types::{
    extensions::CreatorsMut,
    state::{Asset, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, VerifyAccounts},
    require,
};

/// Verifies a creator.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` creator
pub fn process_verify(program_id: &Pubkey, ctx: Context<VerifyAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.creator.is_signer,
        ProgramError::MissingRequiredSignature,
        "missing creator signature"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "invalid asset account owner"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data[0] == Discriminator::Asset.into(),
        ProgramError::UninitializedAccount,
        "Asset account uninitialized"
    );

    let extension = if let Some(creators) = Asset::get_mut::<CreatorsMut>(&mut data) {
        creators
    } else {
        return err!(
            AssetError::ExtensionNotFound,
            "Creators extension not found in asset account"
        );
    };

    // verifies the creator

    let mut found = false;

    extension.creators.iter_mut().for_each(|creator| {
        if creator.address == *ctx.accounts.creator.key {
            creator.verified = true.into();
            found = true;
        }
    });

    if !found {
        return err!(
            ProgramError::InvalidArgument,
            "Creator not found in asset account"
        );
    }

    Ok(())
}
