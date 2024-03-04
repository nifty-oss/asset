use nifty_asset_types::{
    podded::ZeroCopy,
    state::{Asset, Discriminator},
};
use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke, program_error::ProgramError, pubkey::Pubkey,
    rent::Rent, system_instruction, system_program, sysvar::Sysvar,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{Context, CreateAccounts, GroupAccounts},
        MetadataInput,
    },
    require,
};

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
    }

    let mut data = (*ctx.accounts.asset.data).borrow_mut();
    // make sure that the asset is not already initialized since the
    // account might have been created adding extensions
    require!(
        data[0] == Discriminator::Uninitialized.into(),
        AssetError::AlreadyInitialized,
        "asset"
    );

    let asset = Asset::load_mut(&mut data);

    asset.discriminator = Discriminator::Asset;
    asset.standard = args.standard;
    asset.mutable = args.mutable.into();
    asset.holder = *ctx.accounts.holder.key;
    asset.authority = *ctx.accounts.authority.key;
    asset.name = args.name.into();

    let extensions = Asset::get_extensions(&data);

    if !extensions.is_empty() {
        msg!("Asset created with {:?} extension(s)", extensions);
    }

    drop(data);

    // process the group (if there is one)

    if let Some(group) = ctx.accounts.group {
        let accounts = GroupAccounts {
            authority: ctx.accounts.authority,
            asset: ctx.accounts.asset,
            group,
        };
        msg!("Setting group");
        super::group::process_group(
            program_id,
            Context {
                accounts,
                remaining_accounts: ctx.remaining_accounts,
            },
        )?;
    }

    Ok(())
}
