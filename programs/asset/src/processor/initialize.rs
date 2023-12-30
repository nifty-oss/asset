use podded::ZeroCopy;
use solana_program::{
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_memory::sol_memcpy,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
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

    // offset on the account data where the new extension will be written; this will
    // include any padding required to maintain the 8-bytes alignment
    let offset = if ctx.accounts.asset.data_is_empty() {
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

        Asset::LEN
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

            extension.boundary() as usize
        } else {
            Asset::LEN
        }
    };

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

    // initialize the extension

    let (length, data): (u32, &[u8]) = if let Some(data) = &args.data {
        if args.length as usize == data.len() {
            msg!(
                "Initializing extension [{:?}] with instruction data",
                args.extension_type
            );
        }
        (args.length - data.len() as u32, data)
    } else {
        (args.length, &[])
    };

    save_extension_data(&ctx, &args, offset, data)?;

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
    offset: usize,
    data: &[u8],
) -> ProgramResult {
    // make sure the allocated extension maintains the 8-bytes alignment
    let boundary = std::alloc::Layout::from_size_align(
        offset + crate::extensions::Extension::LEN + args.length as usize,
        std::mem::size_of::<u64>(),
    )
    .map_err(|_| AssetError::InvalidAlignment)?
    .pad_to_align()
    .size();

    let extended = offset + crate::extensions::Extension::LEN + data.len();
    let required_rent = Rent::get()?
        .minimum_balance(extended)
        .saturating_sub(ctx.accounts.asset.lamports());

    msg!("Funding {} lamports for account resize", required_rent);

    invoke(
        &system_instruction::transfer(
            ctx.accounts.payer.key,
            ctx.accounts.asset.key,
            required_rent,
        ),
        &[
            ctx.accounts.payer.clone(),
            ctx.accounts.asset.clone(),
            ctx.accounts.system_program.clone(),
        ],
    )?;

    ctx.accounts.asset.realloc(extended, false)?;

    let asset_data = &mut (*ctx.accounts.asset.data).borrow_mut();
    let extension = crate::extensions::Extension::load_mut(
        &mut asset_data[offset..offset + crate::extensions::Extension::LEN],
    );

    extension.set_extension_type(args.extension_type);
    extension.set_length(args.length);
    extension.set_boundary(boundary as u32);

    if !data.is_empty() {
        sol_memcpy(
            &mut asset_data[offset + crate::extensions::Extension::LEN..],
            data,
            data.len(),
        );
    }

    Ok(())
}
