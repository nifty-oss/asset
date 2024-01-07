mod burn;
mod create;
mod delegate;
mod initialize;
mod lock;
mod transfer;
mod unlock;
mod write;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{
            BurnAccounts, CreateAccounts, DelegateAccounts, InitializeAccounts, LockAccounts,
            TransferAccounts, UnlockAccounts, WriteAccounts,
        },
        Instruction,
    },
    state::{Discriminator, State},
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
        Instruction::Burn => {
            msg!("Instruction: Burn");
            burn::process_burn(program_id, BurnAccounts::context(accounts)?)
        }
        Instruction::Create(args) => {
            msg!("Instruction: Create");
            create::process_create(program_id, CreateAccounts::context(accounts)?, args)
        }
        Instruction::Delegate(args) => {
            msg!("Instruction: Delegate");
            delegate::process_delegate(program_id, DelegateAccounts::context(accounts)?, args)
        }
        Instruction::Initialize(args) => {
            msg!("Instruction: Initialize");
            initialize::process_initialize(program_id, InitializeAccounts::context(accounts)?, args)
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
