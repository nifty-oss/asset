mod allocate;
mod burn;
mod close;
mod create;
mod delegate;
mod lock;
mod transfer;
mod unlock;
mod update;
mod write;

use borsh::BorshDeserialize;
use nifty_asset_types::state::{Discriminator, State};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{ProgramResult, MAX_PERMITTED_DATA_INCREASE},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
    sysvar::Sysvar,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{
            AllocateAccounts, BurnAccounts, CloseAccounts, CreateAccounts, DelegateAccounts,
            LockAccounts, TransferAccounts, UnlockAccounts, UpdateAccounts, WriteAccounts,
        },
        Instruction,
    },
};

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    if let Some(account) = is_locked(program_id, accounts) {
        // if we are not unlocking the asset, then we block the instruction
        if !matches!(instruction, Instruction::Unlock) {
            return err!(AssetError::LockedAsset, "Asset \"{}\" is locked", account);
        }
    }

    match instruction {
        Instruction::Allocate(args) => {
            msg!("Instruction: Allocate");
            allocate::process_allocate(program_id, AllocateAccounts::context(accounts)?, args)
        }
        Instruction::Burn => {
            msg!("Instruction: Burn");
            burn::process_burn(program_id, BurnAccounts::context(accounts)?)
        }
        Instruction::Close => {
            msg!("Instruction: Close");
            close::process_close(program_id, CloseAccounts::context(accounts)?)
        }
        Instruction::Create(args) => {
            msg!("Instruction: Create");
            create::process_create(program_id, CreateAccounts::context(accounts)?, args)
        }
        Instruction::Delegate(args) => {
            msg!("Instruction: Delegate");
            delegate::process_delegate(program_id, DelegateAccounts::context(accounts)?, args)
        }
        Instruction::Lock => {
            msg!("Instruction: Lock");
            lock::process_lock(program_id, LockAccounts::context(accounts)?)
        }
        Instruction::Transfer => {
            msg!("Instruction: Transfer");
            transfer::process_transfer(program_id, TransferAccounts::context(accounts)?)
        }
        Instruction::Unlock => {
            msg!("Instruction: Unlock");
            unlock::process_unlock(program_id, UnlockAccounts::context(accounts)?)
        }
        Instruction::Update(args) => {
            msg!("Instruction: Update");
            update::process_update(program_id, UpdateAccounts::context(accounts)?, args)
        }
        Instruction::Write(args) => {
            msg!("Instruction: Write");
            write::process_write(program_id, WriteAccounts::context(accounts)?, args)
        }
    }
}

/// Checks if the instruction's accounts contain a locked asset.
fn is_locked<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo]) -> Option<&'a Pubkey> {
    /// Index of the discriminator byte.
    const DISCRIMINATOR_INDEX: usize = 0;

    /// Index of the asset state byte.
    const STATE_INDEX: usize = 1;

    for account_info in accounts {
        // only considers accounts owned by the program and non-empty
        if account_info.owner == program_id && !account_info.data_is_empty() {
            let data = account_info.data.borrow();
            if (data[DISCRIMINATOR_INDEX] == Discriminator::Asset.into())
                && (data[STATE_INDEX] == State::Locked as u8)
            {
                return Some(account_info.key);
            }
        }
    }

    None
}

#[inline(always)]
fn resize<'a>(
    size: usize,
    account: &'a AccountInfo<'a>,
    payer: Option<&'a AccountInfo<'a>>,
    system_program: Option<&'a AccountInfo<'a>>,
) -> ProgramResult {
    let required = Rent::get()?.minimum_balance(size);

    if account.data_len() > size {
        let delta = account
            .lamports()
            .saturating_sub(if size == 0 { 0 } else { required });

        let payer = payer.ok_or_else(|| {
            msg!("Missing payer account");
            ProgramError::NotEnoughAccountKeys
        })?;

        **payer.try_borrow_mut_lamports()? += delta;
        **account.try_borrow_mut_lamports()? -= delta;
    } else {
        if size.saturating_sub(account.data_len()) > MAX_PERMITTED_DATA_INCREASE {
            return err!(
                ProgramError::AccountDataTooSmall,
                "Cannot increase data size by {} bytes",
                size.saturating_sub(account.data_len())
            );
        }

        let delta = required.saturating_sub(account.lamports());

        if delta > 0 {
            msg!("Funding {} lamports for account resize", delta);

            let payer = payer.ok_or_else(|| {
                msg!("Missing payer account");
                ProgramError::NotEnoughAccountKeys
            })?;

            let system_program = system_program.ok_or_else(|| {
                msg!("Missing system program account");
                ProgramError::NotEnoughAccountKeys
            })?;

            crate::require!(
                payer.is_signer,
                ProgramError::MissingRequiredSignature,
                "payer"
            );

            crate::require!(
                system_program.key == &system_program::ID,
                ProgramError::IncorrectProgramId,
                "system_program"
            );

            invoke(
                &system_instruction::transfer(payer.key, account.key, delta),
                &[account.clone(), payer.clone()],
            )?;
        }
    }

    msg!(
        "Resizing account from {} to {} bytes",
        account.data_len(),
        size
    );

    account.realloc(size, false)?;

    Ok(())
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
