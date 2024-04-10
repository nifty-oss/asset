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

    let asset = Asset::load_mut(&mut asset_data);

    require!(
        asset.group.value().is_none(),
        AssetError::AlreadyInGroup,
        "asset"
    );

    let group = Asset::load_mut(&mut group_data);
    let group_authority = group.authority;

    // authority of the group must match the asset
    require!(
        group.authority == asset.authority,
        AssetError::InvalidAuthority,
        "Group and asset authority mismatch"
    );

    let grouping = if let Some(grouping) = Asset::get_mut::<GroupingMut>(&mut group_data) {
        grouping
    } else {
        return err!(
            AssetError::ExtensionNotFound,
            "Missing required [Grouping] extension"
        );
    };

    // if the signing authority doesn't match the group authority
    if *ctx.accounts.authority.key != group_authority {
        // then the authority must match the grouping delegate
        if let Some(delegate) = grouping.delegate.value() {
            require!(
                *delegate == ctx.accounts.authority.key.into(),
                AssetError::InvalidAuthority,
                "group authority delegate mismatch"
            );
        } else {
            return err!(
                AssetError::InvalidAuthority,
                "missing group authority delegate"
            );
        };
    }

    // group size validation
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
