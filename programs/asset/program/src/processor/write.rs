use nifty_asset_types::state::{Asset, Discriminator};
use solana_program::{
    entrypoint::ProgramResult, msg, program_error::ProgramError, program_memory::sol_memcpy,
    pubkey::Pubkey, system_program,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{Context, WriteAccounts},
        Data,
    },
    processor::resize,
    require,
};

pub(crate) fn process_write(
    program_id: &Pubkey,
    ctx: Context<WriteAccounts>,
    data: Data,
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

    require!(
        !ctx.accounts.asset.data_is_empty(),
        AssetError::InvalidAccountLength,
        "asset"
    );

    let asset_data = (*ctx.accounts.asset.data).borrow();
    // make sure that the asset is not already initialized
    require!(
        asset_data[0] == Discriminator::Uninitialized as u8,
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
        msg!("Extension [{:?}] initialized", current);
    }

    Ok(())
}
