use nifty_asset_types::{
    extensions::GroupingMut,
    podded::{pod::PodOption, ZeroCopy},
    state::{Asset, Discriminator},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, UngroupAccounts},
    require,
};

/// Removes an asset from a group.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[writable]` group
///   2. `[signer]` authority
pub fn process_ungroup(program_id: &Pubkey, ctx: Context<UngroupAccounts>) -> ProgramResult {
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
        "asset"
    );

    // authority must match the group's authority

    let group = Asset::load_mut(&mut group_data);

    require!(
        group.authority == *ctx.accounts.authority.key,
        AssetError::InvalidAuthority,
        "group authority mismatch"
    );

    // asset must be in the group

    let asset = Asset::load_mut(&mut asset_data);

    require!(
        asset.group == PodOption::new(ctx.accounts.group.key.into()),
        ProgramError::InvalidArgument,
        "asset group mismatch"
    );

    // unassign the group to asset and decrease the group size

    let grouping = if let Some(grouping) = Asset::get_mut::<GroupingMut>(&mut group_data) {
        grouping
    } else {
        return err!(
            AssetError::ExtensionNotFound,
            "Missing required [Grouping] extension"
        );
    };

    asset.group = PodOption::new(Pubkey::default().into());
    *grouping.size -= 1;

    Ok(())
}
