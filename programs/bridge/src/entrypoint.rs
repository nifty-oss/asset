use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::{PrintProgramError, ProgramError},
    pubkey::Pubkey,
};

use crate::{error::BridgeError, processor};

entrypoint!(process_instruction);
fn process_instruction<'a>(
    program_id: &'a Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process_instruction(program_id, accounts, instruction_data) {
        match error {
            ProgramError::Custom(_) => error.print::<BridgeError>(),
            _ => {
                solana_program::msg!("⛔️ {} ({:?})", &error.to_string(), &error);
            }
        }
        return Err(error);
    }
    Ok(())
}
