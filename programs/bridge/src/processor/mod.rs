mod bridge;
mod create;

use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey,
    pubkey::Pubkey,
};

use crate::instruction::{
    accounts::{BridgeAccounts, CreateAccounts},
    Instruction,
};

pub static SPL_TOKEN_PROGRAM_IDS: [Pubkey; 2] = [
    pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
    pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"),
];

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        Instruction::Bridge => {
            msg!("Instruction: Bridge");
            bridge::process_bridge(program_id, BridgeAccounts::context(accounts)?)
        }
        Instruction::Create => {
            msg!("Instruction: Create");
            create::process_create(program_id, CreateAccounts::context(accounts)?)
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
