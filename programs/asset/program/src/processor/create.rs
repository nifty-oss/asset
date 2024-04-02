use nifty_asset_types::{
    extensions::{on_create, Extension, ExtensionType, Proxy},
    podded::ZeroCopy,
    state::{Asset, Discriminator, Standard},
};
use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{AllocateAccounts, Context, CreateAccounts, GroupAccounts},
        AllocateInput, MetadataInput,
    },
    require,
};

/// Creates a new asset.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[optional_signer]` authority
///   2. `[]` owner
///   3. `[writable, optional]` group
///   4. `[writable, signer, optional]` payer
///   5. `[optional]` system_program
pub fn process_create(
    program_id: &Pubkey,
    ctx: Context<CreateAccounts>,
    args: MetadataInput,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.asset.is_signer,
        ProgramError::MissingRequiredSignature,
        "asset"
    );

    if ctx.accounts.asset.data_is_empty() {
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

        let space = Asset::LEN
            + if let Some(extensions) = &args.extensions {
                let mut total = 0;

                for extension in extensions {
                    total += std::alloc::Layout::from_size_align(
                        extension.length as usize + Extension::LEN,
                        std::mem::size_of::<u64>(),
                    )
                    .map_err(|_| AssetError::InvalidAlignment)?
                    .pad_to_align()
                    .size();
                }

                total
            } else {
                0
            };

        invoke(
            &system_instruction::create_account(
                payer.key,
                ctx.accounts.asset.key,
                Rent::get()?.minimum_balance(space),
                (space) as u64,
                program_id,
            ),
            &[payer.clone(), ctx.accounts.asset.clone()],
        )?;
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

        let data = &mut (*ctx.accounts.asset.data).borrow_mut();

        // make sure that the asset is not already initialized since the
        // account might have been created adding extensions
        require!(
            data[0] == Discriminator::Uninitialized.into(),
            AssetError::AlreadyInitialized,
            "asset"
        );

        // validates that the last extension is complete
        if let Some((extension, offset)) = Asset::last_extension(data) {
            let extension_type = extension.extension_type();
            let length = extension.length() as usize;

            // validates the last extension found on the account
            on_create(extension_type, &mut data[offset..offset + length]).map_err(|error| {
                msg!("[ERROR] {}", error);
                AssetError::ExtensionDataInvalid
            })?;
        }
    }

    // process extensions (if there are any)

    if let Some(mut extensions) = args.extensions {
        while !extensions.is_empty() {
            super::allocate::process_allocate(
                program_id,
                Context {
                    accounts: AllocateAccounts {
                        asset: ctx.accounts.asset,
                        payer: ctx.accounts.payer,
                        system_program: ctx.accounts.system_program,
                    },
                    remaining_accounts: ctx.remaining_accounts,
                },
                AllocateInput {
                    extension: extensions.swap_remove(0),
                },
            )?;
        }
    }

    let mut data = (*ctx.accounts.asset.data).borrow_mut();
    let asset = Asset::load_mut(&mut data);

    asset.discriminator = Discriminator::Asset;
    asset.standard = args.standard;
    asset.mutable = args.mutable.into();
    asset.owner = *ctx.accounts.owner.key;
    asset.authority = *ctx.accounts.authority.key;
    asset.name = args.name.into();

    let extensions = Asset::get_extensions(&data);

    // make sure that a managed asset is created with the manager
    // extension; and vice versa, a non-managed asset is created
    // without the manager extension
    let has_manager = extensions
        .iter()
        .any(|extension| extension == &ExtensionType::Manager);

    require!(
        matches!(args.standard, Standard::Managed) == has_manager,
        AssetError::ExtensionDataInvalid,
        "{:?} asset + manager extension ({})",
        args.standard,
        has_manager
    );

    // validate the proxy extension if the asset is proxied; or assert
    // that the extension is not present if the asset is not proxied
    let proxy = Asset::get::<Proxy>(&data);

    if matches!(args.standard, Standard::Proxied) {
        require!(
            proxy.is_some(),
            AssetError::ExtensionDataInvalid,
            "missing proxy extension"
        );

        let proxy = proxy.unwrap();

        let derived_key =
            Pubkey::create_program_address(&[proxy.seeds.as_ref(), &[*proxy.bump]], proxy.program)?;

        require!(
            derived_key == *ctx.accounts.asset.key,
            ProgramError::InvalidSeeds,
            "Proxied asset account does not match derived key"
        );
    } else {
        require!(
            proxy.is_none(),
            AssetError::ExtensionDataInvalid,
            "invalid standard for proxy extension"
        );
    }

    if !extensions.is_empty() {
        msg!("Asset created with {:?} extension(s)", extensions);
    }

    drop(data);

    // process the group (if there is one)

    if let Some(group) = ctx.accounts.group {
        msg!("Setting group");
        super::group::process_group(
            program_id,
            Context {
                accounts: GroupAccounts {
                    authority: ctx.accounts.authority,
                    asset: ctx.accounts.asset,
                    group,
                },
                remaining_accounts: ctx.remaining_accounts,
            },
        )?;
    }

    Ok(())
}
