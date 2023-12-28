use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program::invoke_signed,
    program_memory::sol_memcpy, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::{
    err,
    error::DASError,
    extensions::ExtensionType,
    instruction::{
        accounts::{Context, InitializeAccounts},
        Extension,
    },
    require,
    state::{Asset, Discriminator},
};

pub fn process_initialize<'a>(accounts: &'a [AccountInfo<'a>], args: Extension) -> ProgramResult {
    let ctx = InitializeAccounts::context(accounts)?;

    require!(ctx.accounts.mold.is_signer, DASError::MissingSigner);

    // validate account derivation

    let mut seeds = vec![Asset::SEED.as_bytes(), ctx.accounts.mold.key.as_ref()];
    let (derived_key, bump) = Pubkey::find_program_address(&seeds, &crate::ID);

    require!(
        *ctx.accounts.asset.key == derived_key,
        DASError::DeserializationError
    );

    let bump = [bump];
    seeds.push(&bump);

    if ctx.accounts.asset.data_is_empty() {
        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.asset.key,
                Rent::get()?.minimum_balance(Asset::LEN),
                Asset::LEN as u64,
                &crate::ID,
            ),
            &[ctx.accounts.payer.clone(), ctx.accounts.asset.clone()],
            &[&seeds],
        )?;
    } else {
        require!(
            ctx.accounts.asset.data_len() >= Asset::LEN,
            DASError::InvalidAccountLength
        );
    }

    let data = (*ctx.accounts.asset.data).borrow();

    // make sure that the asset is not already initialized
    require!(
        data[0] == Discriminator::Uninitialized as u8,
        DASError::AlreadyInitialized
    );

    // and the account does not have the extension
    require!(
        !Asset::contains(args.extension_type, &data),
        DASError::AlreadyInitialized
    );

    // drop the reference to resize the account
    drop(data);

    if let Some(data) = args.data {
        msg!(
            "Initializing extension '{:?}' with instruction data",
            args.extension_type
        );
        save_extension_data(&ctx, args.extension_type, &data)
    } else if let Some(buffer) = ctx.accounts.buffer {
        msg!(
            "Initializing extension '{:?}' with buffer data",
            args.extension_type
        );
        save_extension_data(&ctx, args.extension_type, &buffer.data.borrow())
    } else {
        err!(DASError::MissingExtensionData)
    }
}

fn save_extension_data(
    ctx: &Context<InitializeAccounts>,
    extension_type: ExtensionType,
    extension: &[u8],
) -> ProgramResult {
    let offset = Asset::allocate(
        ctx.accounts.asset,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        extension_type,
        extension.len(),
    )?;

    let data = &mut (*ctx.accounts.asset.data).borrow_mut();

    sol_memcpy(&mut data[offset..], extension, extension.len());

    Ok(())
}
