use nifty_asset_types::{
    extensions::{validate, Extension},
    podded::{pod::PodBool, ZeroCopy},
    state::{Asset, Discriminator},
};
use solana_program::{
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_memory::{sol_memcpy, sol_memmove},
    pubkey::Pubkey,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{Context, UpdateAccounts},
        UpdateInput,
    },
    processor::resize,
    require,
};

/// Updates an asset's metadata.
///
/// This instruction can update the `name`, `mutable` flag, and any extension data.
/// Note that the update is only possible if the asset is mutable. The updated extension
/// data can be specified as instruction args or through a buffer account. The "layout"
/// of the buffer is the same as as the one created by the `Allocate` instruction.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` authority
///   2. `[writable, signer, optional]` payer
///   3. `[optional]` system_program
#[inline(always)]
pub fn process_update(
    program_id: &Pubkey,
    ctx: Context<UpdateAccounts>,
    args: UpdateInput,
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
        account_data[0] == Discriminator::Asset as u8,
        ProgramError::UninitializedAccount,
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

    // updates the asset:
    //
    //   1. name
    //   2. mutable flag
    //   3. extension

    if let Some(name) = args.name {
        asset.name = name.into();
    }

    if let Some(mutable) = args.mutable {
        asset.mutable = mutable.into();
    }

    // updating an extension is a multi-step process:
    //
    //   1. find the extension to update (it can only update an existing extension)
    //
    //   2. move the data after the extension to a new boundary (this might result
    //      in reducing or extending the size of the extension); the new size should
    //      be within the `MAX_PERMITTED_DATA_INCREASE` limit otherwise this will fail
    //
    //   3. update the extensions' boundaries of any extension after the one being
    //      updated
    //
    //   4. copying the new (updated) extension data
    //
    // at the end of this process, the extension is "loaded" to sanity check that
    // the update was successful
    if let Some(args) = args.extension {
        let mut offset = Asset::LEN;
        let mut extension = None;

        while offset < account_data.len() {
            let current = Extension::load(&account_data[offset..offset + Extension::LEN]);

            if current.extension_type() == args.extension_type {
                extension = Some(current);
                break;
            }

            offset = current.boundary() as usize;
        }

        let extension = extension.ok_or(AssetError::ExtensionNotFound)?;

        // extension data can be specified through a buffer account or
        // instruction args
        let extension_length = if let Some(buffer) = ctx.accounts.buffer {
            require!(
                buffer.owner == program_id,
                ProgramError::IllegalOwner,
                "buffer"
            );

            let extension_data = (*buffer.data).borrow();

            require!(
                extension_data[0] == Discriminator::Uninitialized as u8,
                AssetError::AlreadyInitialized,
                "buffer"
            );

            let (header, _) =
                Asset::first_extension(&extension_data).ok_or(AssetError::ExtensionNotFound)?;

            require!(
                args.extension_type == header.extension_type(),
                ProgramError::InvalidInstructionData,
                "extension type mismatch"
            );

            msg!("Updating extension from buffer account");
            header.length() as usize
        } else {
            msg!("Updating extension from instruction data");
            args.data.as_ref().map_or(0, |d| d.len())
        };

        // determine the new boundary of the extension and any required padding
        // to maintain byte alignment
        let updated_boundary = std::alloc::Layout::from_size_align(
            offset + Extension::LEN + extension_length,
            std::mem::size_of::<u64>(),
        )
        .map_err(|_| AssetError::InvalidAlignment)?
        .pad_to_align()
        .size();
        let boundary = extension.boundary() as usize;

        // calculate the delta between the old and new boundaries to determine
        // by how much the data needs to be moved
        let delta = updated_boundary as i32 - boundary as i32;

        // if the extension is being resized, then the account needs to be resized
        if extension.length() as usize != extension_length {
            let account_boundary = Asset::last_extension(&account_data)
                .ok_or(AssetError::ExtensionNotFound)?
                .0
                .boundary() as usize;
            let resized = account_boundary.saturating_add_signed(delta as isize);
            let bytes_to_move = account_boundary.saturating_sub(boundary);
            // drop the borrow to resize the account
            drop(account_data);

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
                let src_ptr = ptr.add(boundary);
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

            // reborrows the data after the realloc
            account_data = (*ctx.accounts.asset.data).borrow_mut();
        }

        let extension = Extension::load_mut(&mut account_data[offset..offset + Extension::LEN]);

        extension.set_length(extension_length as u32);
        extension.set_boundary(updated_boundary as u32);

        // updates the boundaries of any subsequent extensions
        let mut cursor = updated_boundary;

        while cursor < account_data.len() {
            let extension = Extension::load_mut(&mut account_data[cursor..cursor + Extension::LEN]);
            let boundary = extension.boundary().saturating_add_signed(delta);
            extension.set_boundary(boundary);
            cursor = boundary as usize;
        }

        // copy the updated extension data
        if extension_length > 0 {
            let offset = offset + Extension::LEN;

            if let Some(buffer) = ctx.accounts.buffer {
                let extension_data = (*buffer.data).borrow();
                let slice = Asset::LEN + Extension::LEN;

                sol_memcpy(
                    &mut account_data[offset..],
                    &extension_data[slice..slice + extension_length],
                    extension_length,
                );
            } else if let Some(extension_data) = args.data {
                sol_memcpy(
                    &mut account_data[offset..],
                    &extension_data,
                    extension_length,
                );
            }

            validate(
                args.extension_type,
                &account_data[offset..offset + extension_length],
            )
            .map_err(|error| {
                msg!("Validation error: {:?}", error);
                AssetError::ExtensionDataInvalid
            })?;
        }
    }

    Ok(())
}
