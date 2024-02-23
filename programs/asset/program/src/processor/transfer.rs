use nifty_asset_types::{
    constraints::Context as ConstraintContext,
    extensions::Royalties,
    podded::{
        pod::{Nullable, PodOption},
        ZeroCopy,
    },
    state::{Asset, Delegate, DelegateRole, Discriminator, Standard},
};
use std::ops::Deref;

use crate::{
    error::AssetError,
    instruction::accounts::{Context, TransferAccounts},
    require,
    utils::assert_delegate,
};
use solana_program::{
    entrypoint::ProgramResult,
    instruction::{get_stack_height, TRANSACTION_LEVEL_STACK_HEIGHT},
    program_error::ProgramError,
    pubkey::Pubkey,
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

    // If the asset the asset is part of a collection we need to check if royalties
    // are enabled and if so, if the destination account is allowed to receive the asset.
    if let Some(group) = asset.group.value() {
        if (*group).is_some() {
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
            // let collection_data = (*collection_asset_info.data).borrow();
            require!(
                (*group_asset_info.data).borrow()[0] == Discriminator::Asset as u8,
                AssetError::InvalidGroup,
                "collection account is not initialized"
            );

            // Check if royalties extension is present.
            if let Some(royalties) =
                Asset::get::<Royalties>(&(*group_asset_info.data).borrow()[Asset::LEN..])
            {
                // Check if the recipient is allowed to receive the asset.

                // Wallet-to-wallet transfers between system program accounts are exempt from the royalty check
                // so we need to exclude them.

                // Are we in a CPI? If so, the signer could be a ghost PDA so we cannot prove it's a wallet.
                let is_cpi = get_stack_height() > TRANSACTION_LEVEL_STACK_HEIGHT;

                // Are both the sender and the recipient system program accounts?
                let sender_is_wallet =
                    ctx.accounts.signer.owner == &solana_program::system_program::id();
                let recipient_is_wallet =
                    ctx.accounts.recipient.owner == &solana_program::system_program::id();

                let is_wallet_to_wallet = !is_cpi && sender_is_wallet && recipient_is_wallet;

                if !is_wallet_to_wallet {
                    // We pass in the Constraint context and validate the royalties constraint.
                    royalties.constraint.assertable.assert(&ConstraintContext {
                        asset: group_asset_info,
                        authority: ctx.accounts.signer,
                        recipient: Some(ctx.accounts.recipient),
                    })?;
                }
            }
        }
    }

    // Transfer the asset.
    asset.holder = *ctx.accounts.recipient.key;

    // Clear the delegate.
    asset.delegate = PodOption::new(Delegate::default());

    Ok(())
}
