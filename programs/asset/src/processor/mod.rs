mod create;
mod initialize;

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::DASInstruction;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: DASInstruction = DASInstruction::try_from_slice(instruction_data)?;
    match instruction {
        DASInstruction::Create(args) => {
            msg!("Instruction: Create");
            create::process_create(accounts, args)
        }
        DASInstruction::Initialize(args) => {
            msg!("Instruction: Initialize");
            initialize::process_initialize(accounts, args)
        }
    }
}

#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr ) => {
        if !$constraint {
            return Err($error.into());
        }
    };
}
