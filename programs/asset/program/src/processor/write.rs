use nifty_asset_types::{
    extensions::on_create,
    state::{Asset, Discriminator},
};
use solana_program::{
    entrypoint::ProgramResult, msg, program_error::ProgramError, program_memory::sol_memcpy,
    pubkey::Pubkey, system_program,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{Context, WriteAccounts},
        DataInput,
    },
    processor::resize,
    require,
};

/// Writes data to an extension.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[writable, signer]` payer
///   2. `[optional]` system_program
pub(crate) fn process_write(
    program_id: &Pubkey,
    ctx: Context<WriteAccounts>,
    data: DataInput,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.system_program.key == &system_program::ID,
        ProgramError::IncorrectProgramId,
        "system_program"
    );

    require!(
        ctx.accounts.payer.is_signer,
        ProgramError::MissingRequiredSignature,
        "payer"
    );

    require!(
        ctx.accounts.asset.is_signer,
        ProgramError::MissingRequiredSignature,
        "asset"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let asset_data = (*ctx.accounts.asset.data).borrow();

    require!(
        asset_data.len() >= Asset::LEN,
        AssetError::InvalidAccountLength,
        "asset"
    );

    // make sure that the asset is not already initialized
    require!(
        asset_data[0] == Discriminator::Uninitialized.into(),
        AssetError::AlreadyInitialized,
        "asset"
    );

    // determine the account offset

    let (extension, offset) =
        Asset::last_extension(&asset_data).ok_or(AssetError::ExtensionNotFound)?;
    let current = extension.extension_type();
    let expected = offset.saturating_add(extension.length() as usize);
    let boundary = extension.boundary();
    // the length of the account cannot be larger than the current extension length,
    // otherwise we would be writting data to an extension that does not exist
    if expected < asset_data.len() {
        return err!(AssetError::InvalidAccountLength);
    }

    drop(asset_data);

    let offset = if data.overwrite {
        #[cfg(feature = "logging")]
        msg!("Overwriting extension data");

        resize(
            offset.saturating_add(data.bytes.len()),
            ctx.accounts.asset,
            Some(ctx.accounts.payer),
            Some(ctx.accounts.system_program),
        )?;
        // when overwriting, we start from the beginning of the offset
        offset
    } else {
        #[cfg(feature = "logging")]
        msg!("Appending extension data");

        let offset = ctx.accounts.asset.data_len();
        let mut extended = offset.saturating_add(data.bytes.len());

        if extended == expected {
            extended = boundary as usize;
        }

        resize(
            extended,
            ctx.accounts.asset,
            Some(ctx.accounts.payer),
            Some(ctx.accounts.system_program),
        )?;
        // the offset is the end of the account
        offset
    };

    // copy the data to the buffer

    sol_memcpy(
        &mut ctx.accounts.asset.try_borrow_mut_data().unwrap()[offset..],
        &data.bytes,
        data.bytes.len(),
    );

    if expected > ctx.accounts.asset.data_len() {
        msg!(
            "Writing extension [{:?}] (waiting for {} bytes)",
            current,
            expected.saturating_sub(ctx.accounts.asset.data_len())
        );
    } else {
        let asset_data = (*ctx.accounts.asset.data).borrow();
        let (extension, offset) =
            Asset::last_extension(&asset_data).ok_or(AssetError::ExtensionNotFound)?;

        let extension_type = extension.extension_type();
        let length = extension.length() as usize;

        drop(asset_data);

        // validate the extension data
        let asset_data = &mut (*ctx.accounts.asset.data).borrow_mut();
        on_create(
            extension_type,
            &mut asset_data[offset..offset + length],
            None,
        )
        .map_err(|error| {
            msg!("[ERROR] {}", error);
            AssetError::ExtensionDataInvalid
        })?;

        msg!("Extension [{:?}] initialized", current);
    }

    Ok(())
}
