use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError,
    program_memory::sol_memcpy, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{Context, InitializeAccounts},
        Extension,
    },
    require,
    state::{Asset, Discriminator},
};

pub fn process_initialize(
    program_id: &Pubkey,
    ctx: Context<InitializeAccounts>,
    args: Extension,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.canvas.is_signer,
        ProgramError::MissingRequiredSignature,
        "canvas"
    );

    let mut seeds = vec![Asset::SEED.as_bytes(), ctx.accounts.canvas.key.as_ref()];
    let (derived_key, bump) = Pubkey::find_program_address(&seeds, program_id);

    require!(
        *ctx.accounts.asset.key == derived_key,
        ProgramError::InvalidSeeds,
        "asset"
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
                program_id,
            ),
            &[ctx.accounts.payer.clone(), ctx.accounts.asset.clone()],
            &[&seeds],
        )?;
    } else {
        let data = (*ctx.accounts.asset.data).borrow();

        require!(
            data.len() >= Asset::LEN,
            AssetError::InvalidAccountLength,
            "asset"
        );

        // if there is an extension on the account, need to make sure it has all
        // its data written before initializing a new one
        if let Some((extension, offset)) = Asset::last_extension(&data) {
            require!(
                extension.length() as usize + offset <= data.len(),
                AssetError::IncompleteExtensionData,
                format!(
                    "incomplete [{:?}] extension data",
                    extension.extension_type()
                )
            );
        }
    }

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let data = (*ctx.accounts.asset.data).borrow();

    // make sure that the asset is not already initialized
    require!(
        data[0] == Discriminator::Uninitialized as u8,
        AssetError::AlreadyInitialized,
        "asset"
    );

    // and the account does not have the extension
    require!(
        !Asset::contains(args.extension_type, &data),
        AssetError::AlreadyInitialized,
        "asset"
    );

    // drop the reference to resize the account
    drop(data);

    let length = if let Some(data) = &args.data {
        if args.length as usize == data.len() {
            msg!(
                "Initializing extension [{:?}] with instruction data",
                args.extension_type
            );
        }
        save_extension_data(&ctx, &args, data)?;

        args.length - data.len() as u32
    } else {
        save_extension_data(&ctx, &args, &[])?;
        args.length
    };

    if length > 0 {
        msg!(
            "Initializing extension [{:?}] (waiting for {} bytes)",
            args.extension_type,
            length
        );
    } else {
        msg!("Extension [{:?}] initialized", args.extension_type);
    }

    Ok(())
}

fn save_extension_data(
    ctx: &Context<InitializeAccounts>,
    args: &Extension,
    data: &[u8],
) -> ProgramResult {
    let offset = Asset::allocate(
        ctx.accounts.asset,
        ctx.accounts.payer,
        ctx.accounts.system_program,
        args.into(),
        data.len(),
    )?;

    if !data.is_empty() {
        let account_data = &mut (*ctx.accounts.asset.data).borrow_mut();
        sol_memcpy(&mut account_data[offset..], data, data.len());
    }

    Ok(())
}
