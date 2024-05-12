use nifty_asset_types::{
    extensions::{on_create, Extension, ExtensionType, Proxy},
    podded::ZeroCopy,
    state::{Asset, Discriminator, Standard, DEFAULT_EXTENSION_COUNT},
};
use nitrate::program::system;
use solana_program::{
    entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey, rent::Rent,
    system_program, sysvar::Sysvar,
};

use crate::{
    err,
    error::AssetError,
    instruction::{
        accounts::{Allocate, Context, Create, Group},
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
///   4. `[signer, optional]` group_authority
///   5. `[writable, signer, optional]` payer
///   6. `[optional]` system_program
pub fn process_create(
    program_id: &Pubkey,
    ctx: Context<Create>,
    args: MetadataInput,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.asset.is_signer(),
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
            payer.is_signer(),
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
            system_program.key() == &system_program::ID,
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

        system::create_account(
            payer,
            ctx.accounts.asset,
            Rent::get()?.minimum_balance(space),
            (space) as u64,
            program_id,
        );
    } else {
        require!(
            ctx.accounts.asset.owner() == program_id,
            ProgramError::IllegalOwner,
            "asset"
        );

        require!(
            ctx.accounts.asset.data_len() >= Asset::LEN,
            AssetError::InvalidAccountLength,
            "asset"
        );

        let data = &mut ctx.accounts.asset.try_borrow_mut_data()?;

        // make sure that the asset is not already initialized since the
        // account might have been created adding extensions
        require!(
            data[0] == Discriminator::Uninitialized.into(),
            AssetError::AlreadyInitialized,
            "asset"
        );
    }

    // process extensions (if there are any)

    if let Some(mut extensions) = args.extensions {
        while !extensions.is_empty() {
            super::allocate::process_allocate(
                program_id,
                Context {
                    accounts: Allocate {
                        asset: ctx.accounts.asset,
                        payer: ctx.accounts.payer,
                        system_program: ctx.accounts.system_program,
                    },
                },
                AllocateInput {
                    extension: extensions.swap_remove(0),
                },
            )?;
        }
    }

    // validate the asset extension data (if there is any)

    let mut extensions = Vec::with_capacity(DEFAULT_EXTENSION_COUNT);
    let mut offset = Asset::LEN;

    let mut data = ctx.accounts.asset.try_borrow_mut_data()?;
    let (asset, mut extension_data) = data.split_at_mut(offset);

    while Extension::LEN <= extension_data.len() {
        let (header, remainder) = extension_data.split_at_mut(Extension::LEN);
        // load the extension header
        let extension = Extension::load(header);

        match extension.try_extension_type() {
            Ok(ExtensionType::None) => {
                break;
            }
            Ok(t) => {
                extensions.push(t);
            }
            Err(_) => {
                return err!(AssetError::ExtensionDataInvalid, "invalid extension type");
            }
        }

        // adjust the offset for the extension data slice
        let adjusted = extension.boundary() as usize - (Extension::LEN + offset);
        offset = extension.boundary() as usize;

        if remainder.len() < adjusted {
            return err!(
                AssetError::ExtensionDataInvalid,
                "Invalid extension data (expected {} bytes, got {} bytes)",
                adjusted,
                remainder.len()
            );
        }

        let (current, remainder) = remainder.split_at_mut(adjusted);

        on_create(
            extension.extension_type(),
            &mut current[..extension.length() as usize],
            Some(ctx.accounts.authority.key()),
        )
        .map_err(|error| {
            msg!("[ERROR] {}", error);
            AssetError::ExtensionDataInvalid
        })?;

        extension_data = remainder;
    }

    // creates the asset

    let asset = Asset::load_mut(asset);

    asset.discriminator = Discriminator::Asset;
    asset.standard = args.standard;
    asset.mutable = args.mutable.into();
    asset.owner = *ctx.accounts.owner.key();
    asset.authority = *ctx.accounts.authority.key();
    asset.name = args.name.into();

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
            derived_key == *ctx.accounts.asset.key(),
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

    #[cfg(feature = "logging")]
    if !extensions.is_empty() {
        msg!("Asset created with {:?} extension(s)", extensions);
    }

    drop(data);

    // process the group (if there is one)
    if let Some(group) = ctx.accounts.group {
        #[cfg(feature = "logging")]
        msg!("Setting group");

        super::group::process_group(
            program_id,
            Context {
                accounts: Group {
                    authority: ctx
                        .accounts
                        .group_authority
                        .unwrap_or(ctx.accounts.authority),
                    asset: ctx.accounts.asset,
                    group,
                },
            },
        )?;
    }

    Ok(())
}
