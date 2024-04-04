use nifty_asset_types::{
    extensions::GroupingMut,
    podded::{pod::PodOption, ZeroCopy},
    state::{Asset, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, GroupAccounts},
    require,
};

/// Adds an asset to a group.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[writable]` group
///   2. `[signer]` authority
pub fn process_group(program_id: &Pubkey, ctx: Context<GroupAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.authority.is_signer,
        ProgramError::MissingRequiredSignature,
        "authority"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    require!(
        ctx.accounts.group.owner == program_id,
        ProgramError::IllegalOwner,
        "group"
    );

    let mut asset_data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        asset_data.len() >= Asset::LEN && asset_data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "asset"
    );

    let mut group_data = (*ctx.accounts.group.data).borrow_mut();

    require!(
        group_data.len() >= Asset::LEN && group_data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "group"
    );

    // authority must match the asset's and group's authorities

    let asset = Asset::load_mut(&mut asset_data);

    require!(
        asset.authority == *ctx.accounts.authority.key,
        AssetError::InvalidAuthority,
        "asset authority mismatch"
    );

    require!(
        asset.group.value().is_none(),
        AssetError::AlreadyInGroup,
        "asset"
    );

    let group = Asset::load_mut(&mut group_data);

    require!(
        group.authority == *ctx.accounts.authority.key,
        AssetError::InvalidAuthority,
        "group authority mismatch"
    );

    // group size validation

    let grouping = if let Some(grouping) = Asset::get_mut::<GroupingMut>(&mut group_data) {
        grouping
    } else {
        return err!(
            AssetError::ExtensionNotFound,
            "Missing required [Grouping] extension"
        );
    };

    if let Some(max_size) = grouping.max_size.value() {
        require!(
            *grouping.size < **max_size,
            AssetError::ExtensionDataInvalid,
            "Maximum group size reached"
        );
    }

    // assign the group to asset and increment the group size

    asset.group = PodOption::new(ctx.accounts.group.key.into());
    *grouping.size += 1;

    Ok(())
}
