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

    let group = Asset::load_mut(&mut group_data);
    let asset = Asset::load_mut(&mut asset_data);
    let group_authority = group.authority;

    // asset must be in the group
    require!(
        asset.group == PodOption::new(ctx.accounts.group.key.into()),
        ProgramError::InvalidArgument,
        "asset group mismatch"
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

    // unassign the group to asset and decrease the group size
    asset.group = PodOption::new(Pubkey::default().into());
    *grouping.size -= 1;

    Ok(())
}
