mod burn;
mod create;
mod initialize;
mod transfer;
mod write;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::{
    accounts::{BurnAccounts, CreateAccounts, InitializeAccounts, TransferAccounts, WriteAccounts},
    Instruction,
};

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        Instruction::Burn => {
            msg!("Instruction: Burn");
            burn::process_burn(program_id, BurnAccounts::context(accounts)?)
        }
        Instruction::Create(args) => {
            msg!("Instruction: Create");
            create::process_create(program_id, CreateAccounts::context(accounts)?, args)
        }
        Instruction::Initialize(args) => {
            msg!("Instruction: Initialize");
            initialize::process_initialize(program_id, InitializeAccounts::context(accounts)?, args)
        }
        Instruction::Transfer => {
            msg!("Instruction: Transfer");
            transfer::process_transfer(program_id, TransferAccounts::context(accounts)?)
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
