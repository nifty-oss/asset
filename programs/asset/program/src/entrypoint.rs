use nitrate::{entrypoint, program::AccountInfo};
use solana_program::{
    entrypoint::ProgramResult,
    program_error::{PrintProgramError, ProgramError},
    pubkey::Pubkey,
};

use crate::{error::AssetError, processor};

entrypoint!(process_instruction, 7);

fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        match error {
            ProgramError::Custom(_) => {
                error.print::<AssetError>();
            }
            _ => {
                solana_program::msg!("⛔️ {} ({:?})", &error.to_string(), &error);
            }
        }
        return Err(error);
    }

    Ok(())
}
