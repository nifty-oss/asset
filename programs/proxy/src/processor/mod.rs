mod create;
mod transfer;
mod update;

use borsh::BorshDeserialize;
use nifty_asset_interface::{
    accounts::{TransferAccounts, UpdateAccounts},
    Interface,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

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
            _ => msg!("Unsupported interface instruction"),
        }
    }

    Err(ProgramError::InvalidInstructionData)
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

#[macro_export]
macro_rules! fetch_signer_and_authority {
    ( $signer:ident, $authority:ident, $nifty_asset_program:ident, $ctx:expr, $data:expr ) => {
        // proxy
        let proxy = nifty_asset_interface::state::Asset::get::<
            nifty_asset_interface::extensions::Proxy,
        >($data)
        .unwrap();

        require!(
            proxy.program == &$crate::ID,
            solana_program::program_error::ProgramError::IncorrectProgramId,
            "asset has the wrong proxy program"
        );

        let seeds = *proxy.seeds;
        let $signer = [seeds.as_ref(), &[*proxy.bump]];

        let $authority = if let Some(authority) = proxy.authority.value() {
            Some(*authority)
        } else {
            None
        };

        let $nifty_asset_program = $ctx
            .remaining_accounts
            .first()
            .ok_or(ProgramError::NotEnoughAccountKeys)?;
    };
}
