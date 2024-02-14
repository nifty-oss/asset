use nifty_asset_types::{
    extensions::CreatorsMut,
    state::{Asset, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, UnverifyAccounts},
    require,
};

pub fn process_unverify(program_id: &Pubkey, ctx: Context<UnverifyAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.creator.is_signer,
        ProgramError::MissingRequiredSignature,
        "Missing creator signature"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "Invalid asset account owner"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data[0] == Discriminator::Asset as u8,
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

    // unverifies the creator

    // verifies the creator
    let mut found = false;

    extension.creators.iter_mut().for_each(|creator| {
        if creator.address == *ctx.accounts.creator.key {
            creator.set_verified(false);
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
