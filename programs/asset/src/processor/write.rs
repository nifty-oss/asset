use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke,
    program_error::ProgramError, program_memory::sol_memcpy, pubkey::Pubkey, rent::Rent,
    system_instruction, system_program, sysvar::Sysvar,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{Context, WriteAccounts},
        Data,
    },
    require,
    state::{Asset, Discriminator},
};

pub(crate) fn process_write(
    program_id: &Pubkey,
    ctx: Context<WriteAccounts>,
    data: Data,
) -> ProgramResult {
    // validate account derivation

    require!(
        ctx.accounts.system_program.key == &system_program::ID,
        ProgramError::IncorrectProgramId,
        "system_program"
    );

    require!(
        ctx.accounts.canvas.is_signer,
        ProgramError::MissingRequiredSignature,
        "canvas"
    );

    let (derived_key, _) = Pubkey::find_program_address(
        &[Asset::SEED.as_bytes(), ctx.accounts.canvas.key.as_ref()],
        program_id,
    );

    require!(
        *ctx.accounts.asset.key == derived_key,
        ProgramError::InvalidSeeds,
        "asset"
    );

    if ctx.accounts.asset.data_is_empty() {
        return err!(AssetError::Uninitialized);
    }

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
    // the length of the account cannot be larger than the current extension length,
    // otherwise we would be writting data to an extension that does not exist
    if expected < asset_data.len() {
        return err!(AssetError::InvalidAccountLength);
    }

    drop(asset_data);

    let offset = if data.overwrite {
        msg!("Overwriting [{:?}] extension data", current);
        resize(
            ctx.accounts.asset,
            ctx.accounts.payer,
            offset.saturating_add(data.bytes.len()),
        )?;
        // when overwriting, we start from the beginning of the offset
        offset
    } else {
        msg!("Appending [{:?}] extension data", current);
        let offset = ctx.accounts.asset.data_len();
        resize(
            ctx.accounts.asset,
            ctx.accounts.payer,
            offset.saturating_add(data.bytes.len()),
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

fn resize<'a>(
    account: &'a AccountInfo<'a>,
    payer: &'a AccountInfo<'a>,
    size: usize,
) -> ProgramResult {
    let required = Rent::get()?.minimum_balance(size);

    if account.data_len() > size {
        let delta = account
            .lamports()
            .saturating_sub(if size == 0 { 0 } else { required });
        **payer.try_borrow_mut_lamports()? += delta;
        **account.try_borrow_mut_lamports()? -= delta;
    } else {
        let delta = required.saturating_sub(account.lamports());
        invoke(
            &system_instruction::transfer(payer.key, account.key, delta),
            &[account.clone(), payer.clone()],
        )?;
    }

    account.realloc(size, false)?;

    Ok(())
}
