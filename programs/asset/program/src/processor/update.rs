use nifty_asset_types::{
    extensions::{Extension, ExtensionType},
    state::{Asset, Discriminator},
};
use podded::{pod::PodBool, ZeroCopy};
use solana_program::{
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_memory::{sol_memcpy, sol_memmove},
    pubkey::Pubkey,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{Context, UpdateAccounts},
        UpdateData,
    },
    processor::resize,
    require,
};

#[inline(always)]
pub fn process_update(
    program_id: &Pubkey,
    ctx: Context<UpdateAccounts>,
    args: UpdateData,
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

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data[0] == Discriminator::Asset as u8,
        ProgramError::UninitializedAccount,
        "asset"
    );

    let asset = Asset::load_mut(&mut data);

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

    // update the asset

    if let Some(name) = args.name {
        asset.name = name.into();
    }

    if let Some(mutable) = args.mutable {
        asset.mutable = mutable.into();
    }

    // updating an extension is a multi-step process:
    //
    //   1. find the extension to update
    //
    //   2. move the data after the extension to the new boundary; this might
    //      result in reducing or extending the size of the extension
    //
    //   3. update the extensions' boundaries
    //
    //   4. copying the new data into the extension
    if let Some(args) = args.extension {
        let (length, boundary, offset) = {
            let (current, offset) =
                get_extension(args.extension_type, &data).ok_or(AssetError::ExtensionNotFound)?;
            (current.length(), current.boundary(), offset)
        };

        // determine the new boundary of the extension
        let updated_boundary = std::alloc::Layout::from_size_align(
            offset + Extension::LEN + args.length as usize,
            std::mem::size_of::<u64>(),
        )
        .map_err(|_| AssetError::InvalidAlignment)?
        .pad_to_align()
        .size();
        // determine the delta between the old and new boundaries; this operation is
        // "safe" since boundary values are at most the maximum length of an account
        let delta = updated_boundary as i32 - boundary as i32;

        if length != args.length {
            let resized = data.len().saturating_add_signed(delta as isize);
            let bytes_to_move = data.len().saturating_sub(boundary as usize);
            drop(data);

            if delta > 0 {
                resize(
                    resized,
                    ctx.accounts.asset,
                    ctx.accounts.payer,
                    ctx.accounts.system_program,
                )?;
            }

            unsafe {
                let ptr = ctx.accounts.asset.data.borrow_mut().as_mut_ptr();
                let src_ptr = ptr.add(boundary as usize);
                let dest_ptr = ptr.add(updated_boundary);
                // move the bytes after the extension to the new boundary
                sol_memmove(dest_ptr, src_ptr, bytes_to_move);
            }

            if delta < 0 {
                resize(
                    resized,
                    ctx.accounts.asset,
                    ctx.accounts.payer,
                    ctx.accounts.system_program,
                )?;
            }
        }

        // reborrows the data after the realloc
        data = (*ctx.accounts.asset.data).borrow_mut();

        let extension = Extension::load_mut(&mut data[offset..offset + Extension::LEN]);

        extension.set_length(args.length);
        extension.set_boundary(updated_boundary as u32);
        // updates the boundaries of any subsequent extensions
        update_boundaries(&mut data[updated_boundary..], delta);

        if let Some(extension_data) = args.data {
            sol_memcpy(
                &mut data[offset + Extension::LEN..],
                &extension_data,
                extension_data.len(),
            );
        }
    }

    Ok(())
}

/// Indicates whether the account contains an extension of a given type.
fn get_extension(extension_type: ExtensionType, data: &[u8]) -> Option<(&Extension, usize)> {
    let mut cursor = Asset::LEN;

    while cursor < data.len() {
        let extension = Extension::load(&data[cursor..cursor + Extension::LEN]);

        if extension.extension_type() == extension_type {
            return Some((extension, cursor));
        }

        cursor = extension.boundary() as usize;
    }

    None
}

/// Updates extension boundaries after an extension has been resized.
///
/// This functions assumes that the data is currently at the boundary of
/// the first extension to be updated.
fn update_boundaries(data: &mut [u8], offset: i32) {
    let mut cursor = 0;

    while cursor < data.len() {
        let extension = Extension::load_mut(&mut data[cursor..cursor + Extension::LEN]);
        let boundary = extension.boundary().saturating_add_signed(offset);
        extension.set_boundary(boundary);
        cursor = boundary as usize;
    }
}
