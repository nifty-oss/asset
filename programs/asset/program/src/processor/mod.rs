mod allocate;
mod approve;
mod burn;
mod close;
mod create;
mod group;
mod handover;
mod lock;
mod remove;
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
            AllocateAccounts, ApproveAccounts, BurnAccounts, CloseAccounts, CreateAccounts,
            GroupAccounts, HandoverAccounts, LockAccounts, RemoveAccounts, RevokeAccounts,
            TransferAccounts, UngroupAccounts, UnlockAccounts, UnverifyAccounts, UpdateAccounts,
            VerifyAccounts, WriteAccounts,
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
            allocate::process_allocate(program_id, AllocateAccounts::context(accounts)?, args)
        }
        Instruction::Approve(args) => {
            msg!("Instruction: Approve");
            approve::process_approve(program_id, ApproveAccounts::context(accounts)?, args)
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
        Instruction::Group => {
            msg!("Instruction: Group");
            group::process_group(program_id, GroupAccounts::context(accounts)?)
        }
        Instruction::Handover => {
            msg!("Instruction: Handover");
            handover::process_handover(program_id, HandoverAccounts::context(accounts)?)
        }
        Instruction::Lock => {
            msg!("Instruction: Lock");
            lock::process_lock(program_id, LockAccounts::context(accounts)?)
        }
        Instruction::Remove(args) => {
            msg!("Instruction: Remove");
            remove::process_remove(program_id, RemoveAccounts::context(accounts)?, args)
        }
        Instruction::Revoke(args) => {
            msg!("Instruction: Revoke");
            revoke::process_revoke(program_id, RevokeAccounts::context(accounts)?, args)
        }
        Instruction::Transfer => {
            msg!("Instruction: Transfer");
            transfer::process_transfer(program_id, TransferAccounts::context(accounts)?)
        }
        Instruction::Ungroup => {
            msg!("Instruction: Ungroup");
            ungroup::process_ungroup(program_id, UngroupAccounts::context(accounts)?)
        }
        Instruction::Unlock => {
            msg!("Instruction: Unlock");
            unlock::process_unlock(program_id, UnlockAccounts::context(accounts)?)
        }
        Instruction::Unverify => {
            msg!("Instruction: Unverify");
            unverify::process_unverify(program_id, UnverifyAccounts::context(accounts)?)
        }
        Instruction::Update(args) => {
            msg!("Instruction: Update");
            update::process_update(program_id, UpdateAccounts::context(accounts)?, args)
        }
        Instruction::Verify => {
            msg!("Instruction: Verify");
            verify::process_verify(program_id, VerifyAccounts::context(accounts)?)
        }
        Instruction::Write(args) => {
            msg!("Instruction: Write");
            write::process_write(program_id, WriteAccounts::context(accounts)?, args)
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
        let data = account_info.data.borrow();

        if account_info.owner == program_id
            && !account_info.data_is_empty()
            && data[STANDARD_INDEX] == Standard::Proxied.into()
        {
            require!(
                account_info.is_signer,
                ProgramError::MissingRequiredSignature,
                "proxied asset \"{}\" is not a signer",
                account_info.key
            );
        }
    }

    for account_info in accounts {
        // only considers accounts owned by the program and non-empty
        if account_info.owner == program_id && !account_info.data_is_empty() {
            let data = account_info.data.borrow();
            if data[DISCRIMINATOR_INDEX] == Discriminator::Asset.into()
                && data[STATE_INDEX] == State::Locked.into()
            {
                // any locked asset can be used to determine if the
                // instruction is allowed
                return Ok(Some(account_info.key));
            }
        }
    }

    Ok(None)
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
            msg!("[ERROR] Missing payer account");
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

    #[cfg(feature = "logging")]
    msg!(
        "Resizing account from {} to {} bytes",
        account.data_len(),
        size
    );

    account.realloc(size, false)?;

    Ok(())
}
