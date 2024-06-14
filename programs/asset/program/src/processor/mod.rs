mod allocate;
mod approve;
mod burn;
mod close;
mod create;
mod group;
mod handover;
mod lock;
mod remove;
mod resize;
mod revoke;
mod transfer;
mod ungroup;
mod unlock;
mod unverify;
mod update;
mod verify;
mod write;

use borsh::BorshDeserialize;
use nifty_asset_types::state::{Discriminator, Standard, State};
use nitrate::program::{system, AccountInfo};
use solana_program::{
    entrypoint::{ProgramResult, MAX_PERMITTED_DATA_INCREASE},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_program,
    sysvar::Sysvar,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{
            Allocate, Approve, Burn, Close, Create, Group, Handover, Lock, Remove, Resize, Revoke,
            Transfer, Ungroup, Unlock, Unverify, Update, Verify, Write,
        },
        Instruction,
    },
};

#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    if let Some(account) = validate_access(program_id, accounts)? {
        // check whether the instruction is allowed to proceed
        if matches!(
            instruction,
            Instruction::Approve(_)
                | Instruction::Burn
                | Instruction::Lock
                | Instruction::Revoke(_)
                | Instruction::Transfer
        ) {
            return err!(AssetError::LockedAsset, "Asset \"{}\" is locked", account);
        }
    }

    match instruction {
        Instruction::Allocate(args) => {
            msg!("Instruction: Allocate");
            allocate::process_allocate(program_id, Allocate::context(accounts)?, args)
        }
        Instruction::Approve(args) => {
            msg!("Instruction: Approve");
            approve::process_approve(program_id, Approve::context(accounts)?, args)
        }
        Instruction::Burn => {
            msg!("Instruction: Burn");
            burn::process_burn(program_id, Burn::context(accounts)?)
        }
        Instruction::Close => {
            msg!("Instruction: Close");
            close::process_close(program_id, Close::context(accounts)?)
        }
        Instruction::Create(args) => {
            msg!("Instruction: Create");
            create::process_create(program_id, Create::context(accounts)?, args)
        }
        Instruction::Group => {
            msg!("Instruction: Group");
            group::process_group(program_id, Group::context(accounts)?)
        }
        Instruction::Handover => {
            msg!("Instruction: Handover");
            handover::process_handover(program_id, Handover::context(accounts)?)
        }
        Instruction::Lock => {
            msg!("Instruction: Lock");
            lock::process_lock(program_id, Lock::context(accounts)?)
        }
        Instruction::Remove(args) => {
            msg!("Instruction: Remove");
            remove::process_remove(program_id, Remove::context(accounts)?, args)
        }
        Instruction::Resize(args) => {
            msg!("Instruction: Resize");
            resize::process_resize(program_id, Resize::context(accounts)?, args)
        }
        Instruction::Revoke(args) => {
            msg!("Instruction: Revoke");
            revoke::process_revoke(program_id, Revoke::context(accounts)?, args)
        }
        Instruction::Transfer => {
            msg!("Instruction: Transfer");
            transfer::process_transfer(program_id, Transfer::context(accounts)?)
        }
        Instruction::Ungroup => {
            msg!("Instruction: Ungroup");
            ungroup::process_ungroup(program_id, Ungroup::context(accounts)?)
        }
        Instruction::Unlock => {
            msg!("Instruction: Unlock");
            unlock::process_unlock(program_id, Unlock::context(accounts)?)
        }
        Instruction::Unverify => {
            msg!("Instruction: Unverify");
            unverify::process_unverify(program_id, Unverify::context(accounts)?)
        }
        Instruction::Update(args) => {
            msg!("Instruction: Update");
            update::process_update(program_id, Update::context(accounts)?, args)
        }
        Instruction::Verify => {
            msg!("Instruction: Verify");
            verify::process_verify(program_id, Verify::context(accounts)?)
        }
        Instruction::Write(args) => {
            msg!("Instruction: Write");
            write::process_write(program_id, Write::context(accounts)?, args)
        }
    }
}

#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr, $message:expr ) => {
        if !$constraint {
            solana_program::msg!("Constraint failed: {}", $message);
            return Err($error.into());
        }
    };
    ( $constraint:expr, $error:expr, $message:literal, $($args:tt)+ ) => {
        require!( $constraint, $error, format!($message, $($args)+) );
    };
}

/// Checks if the instruction's accounts contain proxied and locked assets.
///
/// This function will return the key of the locked asset if one is found. An error
/// is raised if a proxied asset is not a signer.
fn validate_access<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo],
) -> Result<Option<&'a Pubkey>, ProgramError> {
    /// Index of the discriminator byte.
    const DISCRIMINATOR_INDEX: usize = 0;
    /// Index of the asset state byte.
    const STATE_INDEX: usize = 1;
    /// Index of the asset state byte.
    const STANDARD_INDEX: usize = 2;

    // we only check the first account of each instruction for `Proxied` assets,
    // since this is the "active" asset on the instruction
    if let Some(account_info) = accounts.first() {
        let data = account_info.try_borrow_data()?;

        if account_info.owner() == program_id
            && !account_info.data_is_empty()
            && data[STANDARD_INDEX] == u8::from(Standard::Proxied)
        {
            require!(
                account_info.is_signer(),
                ProgramError::MissingRequiredSignature,
                "proxied asset \"{}\" is not a signer",
                account_info.key()
            );
        }
    }

    for account_info in accounts {
        // only considers accounts owned by the program and non-empty
        if account_info.owner() == program_id && !account_info.data_is_empty() {
            let data = account_info.try_borrow_data()?;
            if data[DISCRIMINATOR_INDEX] == u8::from(Discriminator::Asset)
                && data[STATE_INDEX] == u8::from(State::Locked)
            {
                // any locked asset can be used to determine if the
                // instruction is allowed
                return Ok(Some(account_info.key()));
            }
        }
    }

    Ok(None)
}

#[inline(always)]
fn resize<'a>(
    size: usize,
    account: &'a AccountInfo,
    payer: Option<&'a AccountInfo>,
    system_program: Option<&'a AccountInfo>,
) -> ProgramResult {
    let required = Rent::get()?.minimum_balance(size);

    if account.data_len() > size {
        let delta =
            account
                .try_borrow_lamports()?
                .saturating_sub(if size == 0 { 0 } else { required });

        let payer = payer.ok_or_else(|| {
            msg!("[ERROR] Missing payer account");
            ProgramError::NotEnoughAccountKeys
        })?;

        *payer.try_borrow_mut_lamports()? += delta;
        *account.try_borrow_mut_lamports()? -= delta;
    } else {
        if size.saturating_sub(account.data_len()) > MAX_PERMITTED_DATA_INCREASE {
            return err!(
                ProgramError::AccountDataTooSmall,
                "Cannot increase data size by {} bytes",
                size.saturating_sub(account.data_len())
            );
        }

        let delta = required.saturating_sub(*account.try_borrow_lamports()?);

        if delta > 0 {
            #[cfg(feature = "logging")]
            msg!("Funding {} lamports for account resize", delta);

            let payer = payer.ok_or_else(|| {
                msg!("[ERROR] Missing payer account");
                ProgramError::NotEnoughAccountKeys
            })?;

            let system_program = system_program.ok_or_else(|| {
                msg!("[ERROR] Missing system program account");
                ProgramError::NotEnoughAccountKeys
            })?;

            crate::require!(
                payer.is_signer(),
                ProgramError::MissingRequiredSignature,
                "payer"
            );

            crate::require!(
                system_program.key() == &system_program::ID,
                ProgramError::IncorrectProgramId,
                "system_program"
            );

            system::transfer(payer, account, delta);
        }
    }

    #[cfg(feature = "logging")]
    msg!(
        "Resizing account from {} to {} bytes",
        account.data_len(),
        size
    );

    account.realloc(size, false)?;

    Ok(())
}
