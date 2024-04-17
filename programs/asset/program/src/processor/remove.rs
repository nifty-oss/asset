use nifty_asset_types::{
    extensions::{Extension, ExtensionType, Grouping},
    podded::{pod::PodBool, ZeroCopy},
    state::{Asset, Discriminator},
};
use solana_program::{
    entrypoint::ProgramResult, msg, program_error::ProgramError, program_memory::sol_memmove,
    pubkey::Pubkey,
};

use crate::{
    err,
    error::AssetError,
    instruction::accounts::{Context, RemoveAccounts},
    processor::resize,
    require,
};

/// Removes an extension from an asset.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` authority
///   2. `[optional]` group
///   3. `[writable]` recipient
#[inline(always)]
pub fn process_remove(
    program_id: &Pubkey,
    ctx: Context<RemoveAccounts>,
    extension_type: ExtensionType,
) -> ProgramResult {
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

    let mut account_data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        account_data.len() >= Asset::LEN && account_data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "asset"
    );

    let asset = Asset::load_mut(&mut account_data);

    require!(
        asset.authority == *ctx.accounts.authority.key,
        AssetError::InvalidAuthority,
        "authority"
    );

    require!(
        <PodBool as Into<bool>>::into(asset.mutable),
        AssetError::ImmutableAsset,
        "asset"
    );

    // removes the extension

    let mut offset = Asset::LEN;
    let mut boundary = None;

    while offset + Extension::LEN < account_data.len() {
        let current = Extension::load(&account_data[offset..offset + Extension::LEN]);

        match current.try_extension_type() {
            Ok(t) if t == extension_type => {
                boundary = Some(current.boundary() as usize);
                break;
            }
            Ok(ExtensionType::None) => break,
            _ => offset = current.boundary() as usize,
        }
    }

    // if the extension is not found, then there is nothing to remove and we
    // signal this by returning an error
    let boundary = boundary.ok_or(AssetError::ExtensionNotFound)?;

    match extension_type {
        ExtensionType::Manager | ExtensionType::Proxy => {
            return err!(
                AssetError::ExtensionDataInvalid,
                "invalid extension type: {:?}",
                extension_type
            );
        }
        ExtensionType::Grouping => {
            let grouping = Asset::get::<Grouping>(&account_data).unwrap();
            // the group must be empty before removing the extension
            require!(
                *grouping.size == 0,
                AssetError::GroupNotEmpty,
                "group not empty ({})",
                grouping.size
            );
        }
        _ => (),
    }
    // drop the account data borrow
    drop(account_data);

    msg!("Removing [{:?}] extension", extension_type);

    // removes the extension by moving the data after the extension to its
    // current position and resizing the account

    let bytes_to_move = ctx.accounts.asset.data_len().saturating_sub(boundary);
    unsafe {
        let ptr = ctx.accounts.asset.data.borrow_mut().as_mut_ptr();
        let src_ptr = ptr.add(boundary);
        let dest_ptr = ptr.add(offset);

        sol_memmove(dest_ptr, src_ptr, bytes_to_move);
    }

    let bytes_to_remove = boundary.saturating_sub(offset);
    resize(
        ctx.accounts
            .asset
            .data_len()
            .saturating_sub(bytes_to_remove),
        ctx.accounts.asset,
        Some(ctx.accounts.recipient),
        None,
    )?;

    Ok(())
}
