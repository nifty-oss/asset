use std::cmp::Ordering;

use nifty_asset_types::{
    extensions::{on_create, on_update, Extension, ExtensionType},
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
    err,
    error::AssetError,
    instruction::{
        accounts::{Context, Update},
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
    ctx: Context<Update>,
    args: UpdateInput,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.authority.is_signer(),
        ProgramError::MissingRequiredSignature,
        "authority"
    );

    require!(
        ctx.accounts.asset.owner() == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let mut account_data = ctx.accounts.asset.try_borrow_mut_data()?;

    require!(
        account_data.len() >= Asset::LEN && account_data[0] == Discriminator::Asset.into(),
        AssetError::Uninitialized,
        "asset"
    );

    let asset = Asset::load_mut(&mut account_data);

    require!(
        asset.authority == *ctx.accounts.authority.key(),
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

    // creating/updating an extension is a multi-step process:
    //
    //   1. find the extension to update or determine that it is a new one
    //
    //   2. move the data after the extension to a new boundary (this might result
    //      in reducing or extending the size of the extension); the new size should
    //      be within the `MAX_PERMITTED_DATA_INCREASE` limit otherwise this will fail
    //      (only happens when the extension is being updated)
    //
    //   3. update the extensions' boundaries of any extension after the one being
    //      updated (only happens when the extension is being updated)
    //
    //   4. copying the new/updated extension data
    //
    // at the end of this process, the extension is "loaded" to sanity check that
    // the update was successful
    if args.extension.is_some() || ctx.accounts.buffer.is_some() {
        // extension data can be specified through a buffer account or
        // instruction args, but not both
        require!(
            args.extension.is_some() != ctx.accounts.buffer.is_some(),
            ProgramError::InvalidInstructionData,
            "only specify instruction args or buffer account"
        );

        let extension_type = if let Some(buffer) = ctx.accounts.buffer {
            require!(
                buffer.owner() == program_id,
                ProgramError::IllegalOwner,
                "buffer"
            );

            let extension_data = buffer.try_borrow_data()?;

            require!(
                extension_data.len() >= Asset::LEN + Extension::LEN,
                AssetError::InvalidAccountLength,
                "buffer"
            );

            require!(
                extension_data[0] == Discriminator::Uninitialized.into(),
                AssetError::AlreadyInitialized,
                "buffer"
            );

            let (header, _) =
                Asset::first_extension(&extension_data).ok_or(AssetError::ExtensionNotFound)?;

            header.extension_type()
        } else if let Some(args) = &args.extension {
            args.extension_type
        } else {
            // sanity check: this should not happen since we already checked that we either
            // have extension args or a buffer account
            return Err(ProgramError::InvalidInstructionData);
        };

        let mut offset = Asset::LEN;
        // extension details:
        //   - current length
        //   - account boundary
        //   - update flag (false when creating the extension)
        let mut extension = None;

        while offset < account_data.len() {
            let current = Extension::load(&account_data[offset..offset + Extension::LEN]);

            match current.try_extension_type() {
                Ok(t) if t == extension_type => {
                    extension =
                        Some((current.length() as usize, current.boundary() as usize, true));
                    break;
                }
                Ok(ExtensionType::None) => break,
                _ => offset = current.boundary() as usize,
            }
        }

        let (current_length, boundary, update) = if let Some(extension) = extension {
            extension
        } else {
            (0, offset, false)
        };

        let (extension_length, extension_data) = if let Some(buffer) = ctx.accounts.buffer {
            let extension_data = buffer.try_borrow_data()?;
            let (header, _) =
                Asset::first_extension(&extension_data).ok_or(AssetError::ExtensionNotFound)?;

            require!(
                extension_type == header.extension_type(),
                ProgramError::InvalidInstructionData,
                "extension type mismatch"
            );

            require!(
                buffer.data_len() >= header.boundary() as usize,
                AssetError::ExtensionDataInvalid,
                "invalid extension data (expected {} bytes, got {} bytes)",
                header.boundary(),
                buffer.data_len()
            );

            let header_length = header.length() as usize;

            drop(extension_data);

            let start = offset + Extension::LEN;
            // validate the extension data
            validate(
                extension_type,
                &mut buffer.try_borrow_mut_data()?[Asset::LEN..Asset::LEN + header_length],
                if update {
                    Some(&mut account_data[start..start + current_length])
                } else {
                    None
                },
                ctx.accounts.authority.key(),
            )?;

            #[cfg(feature = "logging")]
            msg!("Updating extension from buffer account");

            (header_length, None)
        } else if let Some(args) = args.extension {
            let length = if let Some(mut extension_data) = args.data {
                let start = offset + Extension::LEN;
                // validate the extension data
                validate(
                    extension_type,
                    extension_data.as_mut_slice(),
                    if update {
                        Some(&mut account_data[start..start + current_length])
                    } else {
                        None
                    },
                    ctx.accounts.authority.key(),
                )?;

                (extension_data.len(), Some(extension_data))
            } else {
                // extension does not have any data
                (0, None)
            };

            #[cfg(feature = "logging")]
            msg!("Updating extension from instruction data");

            // sanity check: did we receive the correct extension length?
            match length.0.cmp(&(args.length as usize)) {
                Ordering::Less => {
                    return err!(
                        AssetError::ExtensionDataInvalid,
                        "invalid extension data (expected {} bytes, got {} bytes)",
                        args.length,
                        length.0
                    );
                }
                Ordering::Greater => {
                    return err!(
                        AssetError::ExtensionLengthInvalid,
                        "extension length mismatch"
                    );
                }
                Ordering::Equal => (),
            }

            length
        } else {
            // sanity check: this should not happen since we already checked that we either
            // have extension args or a buffer account
            return Err(ProgramError::InvalidInstructionData);
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

        // calculate the delta between the old and new boundaries to determine
        // by how much the data needs to be moved
        let delta = updated_boundary as i32 - boundary as i32;

        if !update {
            // drop the borrow to resize the account
            drop(account_data);

            resize(
                updated_boundary,
                ctx.accounts.asset,
                ctx.accounts.payer,
                ctx.accounts.system_program,
            )?;

            // reborrows the data after the realloc
            account_data = ctx.accounts.asset.try_borrow_mut_data()?;
        }
        // if the extension is being resized, then the account needs to be resized
        else if current_length != extension_length {
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
                let ptr = ctx.accounts.asset.unchecked_borrow_mut_data().as_mut_ptr();
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
            account_data = ctx.accounts.asset.try_borrow_mut_data()?;
        }

        let extension = Extension::load_mut(&mut account_data[offset..offset + Extension::LEN]);

        extension.set_extension_type(extension_type);
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
                let extension_data = buffer.try_borrow_data()?;
                let slice = Asset::LEN + Extension::LEN;

                sol_memcpy(
                    &mut account_data[offset..],
                    &extension_data[slice..slice + extension_length],
                    extension_length,
                );
            } else if let Some(extension_data) = extension_data {
                sol_memcpy(
                    &mut account_data[offset..],
                    &extension_data,
                    extension_length,
                );
            }
        }
    }

    Ok(())
}

/// Validates the extension data.
///
/// This function is used to validate the extension data when creating or updating an extension.
/// If the `current_data` is provided, then the extension is being updated, otherwise it is being
/// created.
#[inline(always)]
fn validate(
    extension_type: ExtensionType,
    input_data: &mut [u8],
    current_data: Option<&mut [u8]>,
    authority: &Pubkey,
) -> Result<(), AssetError> {
    if let Some(current_data) = current_data {
        on_update(extension_type, current_data, input_data, Some(authority)).map_err(|error| {
            msg!("[ERROR] {}", error);
            AssetError::ExtensionDataInvalid
        })
    } else {
        match extension_type {
            ExtensionType::None | ExtensionType::Manager | ExtensionType::Proxy => {
                err!(
                    AssetError::ExtensionDataInvalid,
                    "invalid extension type: {:?}",
                    extension_type
                )
            }
            _ => on_create(extension_type, input_data, Some(authority)).map_err(|error| {
                msg!("[ERROR] {}", error);
                AssetError::ExtensionDataInvalid
            }),
        }
    }
}
