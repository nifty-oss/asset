use borsh::BorshDeserialize;
use nifty_asset_interface::{
    accounts::TransferAccounts,
    extensions::{Proxy, ProxyBuilder},
    instructions::{CreateCpiBuilder, TransferCpiBuilder},
    state::Asset,
    types::{ExtensionInput, ExtensionType, Standard},
    Interface,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::{
    accounts::{Context, CreateAccounts},
    Instruction,
};

pub fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    // we first try to match the instruction on the Nifty Asset interface
    if let Ok(instruction) = Interface::try_from_slice(instruction_data) {
        msg!("Interface: {:?}", instruction);

        if let Interface::Transfer = instruction {
            return process_transfer(program_id, TransferAccounts::context(accounts)?);
        }
    }

    let instruction: Instruction = Instruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        Instruction::Create => {
            msg!("Instruction: Create");
            process_create(program_id, CreateAccounts::context(accounts)?)
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

fn process_create(program_id: &Pubkey, ctx: Context<CreateAccounts>) -> ProgramResult {
    // proxy

    let (derived_key, bump) =
        Pubkey::find_program_address(&[ctx.accounts.stub.key.as_ref()], program_id);

    require!(
        derived_key == *ctx.accounts.asset.key,
        ProgramError::InvalidSeeds,
        "asset"
    );

    let signer = [ctx.accounts.stub.key.as_ref(), &[bump]];

    let mut proxy = ProxyBuilder::default();
    proxy.set(
        program_id,
        &ctx.accounts.stub.key.to_bytes(),
        bump,
        ctx.accounts.owner.key,
    );

    CreateCpiBuilder::new(ctx.accounts.nifty_asset_program)
        .asset(ctx.accounts.asset)
        .authority(ctx.accounts.asset, false)
        .owner(ctx.accounts.owner)
        .payer(ctx.accounts.payer)
        .system_program(ctx.accounts.system_program)
        .name(String::from("Proxied"))
        .standard(Standard::Proxied)
        .extensions(vec![ExtensionInput {
            extension_type: ExtensionType::Proxy,
            length: proxy.len() as u32,
            data: Some(proxy.data()),
        }])
        .invoke_signed(&[&signer])
}

fn process_transfer<'a>(
    program_id: &Pubkey,
    ctx: nifty_asset_interface::accounts::Context<'a, TransferAccounts<'a>>,
) -> ProgramResult {
    // proxy

    let data = (*ctx.accounts.asset.data).borrow();
    let proxy = Asset::get::<Proxy>(&data).unwrap();

    require!(
        proxy.program == program_id,
        ProgramError::IncorrectProgramId,
        "asset has the wrong proxy program"
    );

    let seeds = *proxy.seeds;
    let signer = [seeds.as_ref(), &[*proxy.bump]];

    drop(data);

    // CPI into the Nifty Asset program

    let nifty_asset_program = ctx
        .remaining_accounts
        .first()
        .ok_or(ProgramError::NotEnoughAccountKeys)?;

    TransferCpiBuilder::new(nifty_asset_program)
        .asset(ctx.accounts.asset)
        .signer(ctx.accounts.signer)
        .recipient(ctx.accounts.recipient)
        .invoke_signed(&[&signer])
}
