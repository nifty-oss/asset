use nifty_asset_types::{
    extensions::{on_create, Extension},
    podded::ZeroCopy,
    state::{Asset, Discriminator},
};
use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke, program_error::ProgramError,
    program_memory::sol_memcpy, pubkey::Pubkey, rent::Rent, system_instruction, system_program,
    sysvar::Sysvar,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{AllocateAccounts, Context},
        AllocateInput,
    },
    processor::resize,
    require,
};

/// Allocates an extension into an uninitialized asset (buffer) account.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[writable, signer, optional]` payer
///   2. `[optional]` system_program
#[inline(always)]
pub fn process_allocate(
    program_id: &Pubkey,
    ctx: Context<AllocateAccounts>,
    args: AllocateInput,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.asset.is_signer,
        ProgramError::MissingRequiredSignature,
        "asset"
    );

    // offset on the account data where the new extension will be written; this will
    // include any padding required to maintain the 8-bytes alignment
    let offset = if ctx.accounts.asset.data_is_empty() {
        let payer = {
            require!(
                ctx.accounts.payer.is_some(),
                ProgramError::NotEnoughAccountKeys,
                "payer"
            );

            ctx.accounts.payer.unwrap()
        };

        require!(
            payer.is_signer,
            ProgramError::MissingRequiredSignature,
            "payer"
        );

        let system_program = {
            require!(
                ctx.accounts.system_program.is_some(),
                ProgramError::NotEnoughAccountKeys,
                "system_program"
            );

            ctx.accounts.system_program.unwrap()
        };

        require!(
            system_program.key == &system_program::ID,
            ProgramError::IncorrectProgramId,
            "system_program"
        );

        invoke(
            &system_instruction::create_account(
                payer.key,
                ctx.accounts.asset.key,
                Rent::get()?.minimum_balance(Asset::LEN),
                Asset::LEN as u64,
                program_id,
            ),
            &[payer.clone(), ctx.accounts.asset.clone()],
        )?;

        Asset::LEN
    } else {
        require!(
            ctx.accounts.asset.owner == program_id,
            ProgramError::IllegalOwner,
            "asset"
        );

        require!(
            ctx.accounts.asset.data_len() >= Asset::LEN,
            AssetError::InvalidAccountLength,
            "asset"
        );

        let data = (*ctx.accounts.asset.data).borrow();
        // if there is an extension on the account, need to make sure it has all
        // its data written before initializing a new one
        if let Some((extension, offset)) = Asset::last_extension(&data) {
            require!(
                extension.length() as usize + offset <= data.len(),
                AssetError::IncompleteExtensionData,
                "incomplete [{:?}] extension data",
                extension.extension_type()
            );

            extension.boundary() as usize
        } else {
            Asset::LEN
        }
    };

    let data = (*ctx.accounts.asset.data).borrow();
    // make sure that the asset is not already initialized
    require!(
        data[0] == Discriminator::Uninitialized.into(),
        AssetError::AlreadyInitialized,
        "asset"
    );
    // and the account does not have the extension
    require!(
        !Asset::contains(args.extension.extension_type, &data),
        AssetError::AlreadyInitialized,
        "extension [{:?}] already initialized",
        args.extension.extension_type
    );

    // drop the reference to resize the account
    drop(data);

    // initialize the extension

    let (length, data): (u32, &[u8]) = if let Some(data) = &args.extension.data {
        #[cfg(feature = "logging")]
        if args.extension.length as usize == data.len() {
            msg!(
                "Initializing extension [{:?}] with instruction data",
                args.extension.extension_type
            );
        }
        (args.extension.length - data.len() as u32, data)
    } else {
        (args.extension.length, &[])
    };

    save_extension_data(&ctx, &args.extension, offset, data)?;

    if length > 0 {
        msg!(
            "Initializing extension [{:?}] (waiting for {} bytes)",
            args.extension.extension_type,
            length
        );
    } else {
        let asset_data = (*ctx.accounts.asset.data).borrow();
        let (extension, offset) =
            Asset::last_extension(&asset_data).ok_or(AssetError::ExtensionNotFound)?;

        let extension_type = extension.extension_type();
        let length = extension.length() as usize;

        drop(asset_data);

        // validate the extension data
        let asset_data = &mut (*ctx.accounts.asset.data).borrow_mut();
        on_create(extension_type, &mut asset_data[offset..offset + length]).map_err(|error| {
            msg!("[ERROR] {}", error);
            AssetError::ExtensionDataInvalid
        })?;

        msg!(
            "Extension [{:?}] initialized",
            args.extension.extension_type
        );
    }

    Ok(())
}

fn save_extension_data(
    ctx: &Context<AllocateAccounts>,
    args: &crate::instruction::ExtensionInput,
    offset: usize,
    data: &[u8],
) -> ProgramResult {
    // make sure the allocated extension maintains the 8-bytes alignment
    let boundary = std::alloc::Layout::from_size_align(
        offset + Extension::LEN + args.length as usize,
        std::mem::size_of::<u64>(),
    )
    .map_err(|_| AssetError::InvalidAlignment)?
    .pad_to_align()
    .size();
    // there are two aspects to consider when resizing the account:
    //
    //   1. if the instruction data length is the same as the extension length,
    //      then we can use the extension boundary as the account length
    //
    //   2. when we are expecting more data, we need to make sure the account
    //      does not have any padding (extra bytes)
    let (extended, partial) = if args.length == data.len() as u32 {
        (boundary, false)
    } else {
        (offset + Extension::LEN + data.len(), true)
    };

    if extended > ctx.accounts.asset.data_len() || partial {
        resize(
            extended,
            ctx.accounts.asset,
            ctx.accounts.payer,
            ctx.accounts.system_program,
        )?;
    }

    let asset_data = &mut (*ctx.accounts.asset.data).borrow_mut();
    let extension = Extension::load_mut(&mut asset_data[offset..offset + Extension::LEN]);

    extension.set_extension_type(args.extension_type);
    extension.set_length(args.length);
    extension.set_boundary(boundary as u32);

    if !data.is_empty() {
        sol_memcpy(&mut asset_data[offset + Extension::LEN..], data, data.len());
    }

    Ok(())
}
