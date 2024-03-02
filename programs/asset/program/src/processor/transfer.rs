use nifty_asset_types::{
    constraints::{Assertion, Context as ConstraintContext},
    extensions::Royalties,
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
        "missing a signer"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    // Must be an initialized asset.
    require!(
        data[0] == Discriminator::Asset as u8,
        AssetError::Uninitialized,
        "unitialized asset"
    );

    // First we check if the asset itself has the royalties extension, and validate the constraint.
    let royalties_checked = process_royalties!(ctx, &data);

    let asset = Asset::load_mut(&mut data);

    // Cannot transfer soulbound assets.
    require!(
        asset.standard != Standard::Soulbound,
        AssetError::CannotTransferSoulbound,
        "soulbound asset"
    );

    let is_holder = asset.holder == *ctx.accounts.signer.key;

    let is_delegate =
        assert_delegate(asset, ctx.accounts.signer.key, DelegateRole::Transfer).is_ok();

    // Signing account must be holder or a transfer delegate.
    require!(
        is_holder || is_delegate,
        AssetError::InvalidTransferAuthority,
        "not a holder or transfer delegate"
    );

    // Self transfer short-circuits so as not to clear the delegate.
    if asset.holder == *ctx.accounts.recipient.key {
        return Ok(());
    }

    // If the asset the asset is part of a group we need to check if royalties
    // are enabled and if so, if the destination account is allowed to receive the asset.
    if let Some(group) = asset.group.value() {
        // If royalties were not checked yet, we need to check them now.
        if (*group).is_some() && !royalties_checked {
            // We need collection asset account to be provided.
            require!(
                ctx.accounts.group_asset.is_some(),
                AssetError::InvalidGroup,
                "asset is part of a group but no collection account was provided"
            );

            let group_asset_info = ctx.accounts.group_asset.unwrap();

            // Collection asset account must be owned by the program.
            require!(
                group_asset_info.owner == program_id,
                AssetError::InvalidGroup,
                "collection account is not owned by the program"
            );

            // Collection asset account must match the group.
            require!(
                (*group).deref() == ctx.accounts.group_asset.unwrap().key,
                AssetError::InvalidGroup,
                "collection account does not match the group"
            );

            // Collection asset account must be initialized.
            require!(
                (*group_asset_info.data).borrow()[0] == Discriminator::Asset as u8,
                AssetError::InvalidGroup,
                "collection account is not initialized"
            );

            // Check if royalties extension is present on the group asset and validate the constraint.
            process_royalties!(ctx, &(*group_asset_info.data).borrow());
        }
    }

    // Transfer the asset.
    asset.holder = *ctx.accounts.recipient.key;

    // Clear the delegate.
    asset.delegate = PodOption::new(Delegate::default());

    Ok(())
}
