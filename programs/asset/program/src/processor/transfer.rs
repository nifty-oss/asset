use nifty_asset_types::{
    constraints::{Assertion, Context as ConstraintContext},
    extensions::{Extension, Manager, Royalties},
    podded::{
        pod::{Nullable, PodOption},
        ZeroCopy,
    },
    state::{Asset, Delegate, DelegateRole, Discriminator, Standard},
};
use std::ops::Deref;

use solana_program::{
    entrypoint::ProgramResult,
    instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, TransferAccounts},
    process_royalties, require,
    utils::assert_delegate,
};

/// Transfers ownership of the aseet to a new public key.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` signer
///   2. `[]` recipient
///   3. `[optional]` group_asset
pub fn process_transfer(program_id: &Pubkey, ctx: Context<TransferAccounts>) -> ProgramResult {
    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    // Account must be a signer.
    require!(
        ctx.accounts.signer.is_signer,
        ProgramError::MissingRequiredSignature,
        "signer"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    // Must be an initialized asset.
    require!(
        data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "unitialized asset"
    );

    // First we check if the asset itself has the royalties extension, and validate the constraint.
    let royalties_checked = process_royalties!(ctx, &data);

    let (asset, extensions) = data.split_at_mut(Asset::LEN);
    let asset = Asset::load_mut(asset);

    // Cannot transfer soulbound assets.
    require!(
        asset.standard != Standard::Soulbound,
        AssetError::CannotTransferSoulbound,
        "soulbound asset"
    );

    let is_allowed = asset.owner == *ctx.accounts.signer.key
        || assert_delegate(
            &[
                asset.delegate.value(),
                Extension::get::<Manager>(extensions).map(|s| s.delegate),
            ],
            ctx.accounts.signer.key,
            DelegateRole::Transfer,
        )
        .is_ok();

    // Signing account must be owner or a transfer delegate.
    require!(
        is_allowed,
        AssetError::InvalidTransferAuthority,
        "not an owner or transfer delegate"
    );

    // Self transfer short-circuits so as not to clear the delegate.
    if asset.owner == *ctx.accounts.recipient.key {
        return Ok(());
    }

    // If the asset the asset is part of a group we need to check if royalties
    // are enabled and if so, if the destination account is allowed to receive the asset.
    if let Some(group) = asset.group.value() {
        // If royalties were not checked yet, we need to check them now.
        if group.is_some() && !royalties_checked {
            // We need group asset account to be provided.
            require!(
                ctx.accounts.group_asset.is_some(),
                ProgramError::NotEnoughAccountKeys,
                "asset is part of a group but no group account was provided"
            );

            let group_asset_info = ctx.accounts.group_asset.unwrap();

            // Group asset account must be owned by the program.
            require!(
                group_asset_info.owner == program_id,
                AssetError::InvalidGroup,
                "group account is not owned by the program"
            );

            // Group asset account must match the asset group.
            require!(
                group.deref() == group_asset_info.key,
                AssetError::InvalidGroup,
                "group account does not match the asset group"
            );

            // Group asset account must be initialized.
            require!(
                (*group_asset_info.data).borrow()[0] == Discriminator::Asset.into(),
                AssetError::InvalidGroup,
                "group account is not initialized"
            );

            // Check if royalties extension is present on the group asset and validate the constraint.
            process_royalties!(ctx, &(*group_asset_info.data).borrow());
        }
    }

    // Transfer the asset.
    asset.owner = *ctx.accounts.recipient.key;

    // Clear the delegate.
    asset.delegate = PodOption::new(Delegate::default());

    Ok(())
}
