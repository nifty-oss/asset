mod create;
mod transfer;
mod update;

use borsh::BorshDeserialize;
use nifty_asset_interface::{
    accounts::{TransferAccounts, UpdateAccounts},
    Interface,
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::{accounts::CreateAccounts, Instruction};

const IMAGE: &str = include_str!("../image.svg");

const CONTENT_TYPE: &str = "image/svg+xml";

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    // we first try to match the instruction on the `Proxy` program
    // interface since they do not overlap

    let instruction = Instruction::try_from_slice(instruction_data);

    if let Ok(Instruction::Create(metadata)) = instruction {
        msg!("Instruction: Create");
        return create::process_create(program_id, CreateAccounts::context(accounts)?, metadata);
    }

    // if it is not an instruction from the `Proxy` program, we try to match it
    // on the Nifty Asset interface

    if let Ok(instruction) = Interface::try_from_slice(instruction_data) {
        match instruction {
            Interface::Transfer => {
                msg!("Interface: Transfer");
                return transfer::process_transfer(
                    program_id,
                    TransferAccounts::context(accounts)?,
                );
            }
            Interface::Update(input) => {
                msg!("Interface: Update");
                return update::process_update(
                    program_id,
                    UpdateAccounts::context(accounts)?,
                    input,
                );
            }
            // we can block instructions that are not supported by the proxy
            // by returning an error here
            _ => (),
        }
    }

    Interface::process_instruction(&crate::ID, accounts, instruction_data)
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
