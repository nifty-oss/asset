use nifty_asset_types::{
    extensions::{Grouping, GroupingMut, Manager},
    podded::ZeroCopy,
    state::{Asset, DelegateRole, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};
use std::ops::Deref;

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{BurnAccounts, Context},
    require,
    utils::{assert_delegate, close_program_account},
};

/// Burns an asset.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[writable, signer]` signer
///   2. `[writable, optional]` recipient
///   3. `[writable, optional]` group
pub fn process_burn(program_id: &Pubkey, ctx: Context<BurnAccounts>) -> ProgramResult {
    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    require!(
        ctx.accounts.signer.is_signer,
        ProgramError::MissingRequiredSignature,
        "missing required signer"
    );

    let data = ctx.accounts.asset.try_borrow_data()?;

    // Must be an initialized asset.
    require!(
        data.len() >= Asset::LEN && data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "asset"
    );

    let (asset, extensions) = data.split_at(Asset::LEN);
    let asset = Asset::load(asset);

    // Validate the signer is the owner or the burn delegate.
    let is_allowed = asset.owner == *ctx.accounts.signer.key
        || assert_delegate(
            &[
                asset.delegate.value(),
                Asset::get::<Manager>(extensions).map(|s| s.delegate),
            ],
            ctx.accounts.signer.key,
            DelegateRole::Burn,
        )
        .is_ok();

    require!(
        is_allowed,
        AssetError::InvalidBurnAuthority,
        "not an owner or burn delegate"
    );

    // decrease the group size (if necessary)

    if let Some(group) = asset.group.value() {
        // if the asset is part of a group, we require the group account to be present
        // to decrease the group size
        let group_asset = ctx.accounts.group.ok_or_else(|| {
            msg!("[ERROR] Missing group account");
            ProgramError::NotEnoughAccountKeys
        })?;

        require!(
            group.deref() == group_asset.key,
            ProgramError::InvalidArgument,
            "group mismatch"
        );

        let mut group_data = group_asset.data.borrow_mut();

        // sanity check: this should not happen since the asset is referencing the group
        let grouping = if let Some(grouping) = Asset::get_mut::<GroupingMut>(&mut group_data) {
            grouping
        } else {
            return err!(
                AssetError::ExtensionNotFound,
                "Missing required [Grouping] extension"
            );
        };
        // (safely) decrease the group size
        *grouping.size = grouping
            .size
            .checked_sub(1)
            .ok_or(ProgramError::InvalidAccountData)?;
    }

    // if the asset itself is a group, we require it to be empty
    if let Some(grouping) = Asset::get::<Grouping>(&data) {
        require!(
            *grouping.size == 0,
            AssetError::GroupNotEmpty,
            "asset represents a non-empty group"
        );
    }

    // drop asset account reference
    drop(data);

    let recipient = ctx.accounts.recipient.unwrap_or(ctx.accounts.signer);
    close_program_account(ctx.accounts.asset, recipient)
}
