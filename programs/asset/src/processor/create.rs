use podded::{types::POD_TRUE, ZeroCopy};
use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{Context, CreateAccounts},
        Metadata,
    },
    require,
    state::{Asset, Discriminator},
};

pub fn process_create(
    program_id: &Pubkey,
    ctx: Context<CreateAccounts>,
    args: Metadata,
) -> ProgramResult {
    // validate account derivation

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
        data[0] == Discriminator::Uninitialized as u8,
        AssetError::AlreadyInitialized,
        "asset"
    );

    let mut asset = Asset::load_mut(&mut data);

    asset.discriminator = Discriminator::Asset;
    asset.bump = bump[0];
    asset.mutable = POD_TRUE;
    asset.holder = *ctx.accounts.holder.key;
    asset.authority = *ctx.accounts.authority.key;
    asset.name = args.name.into();
    asset.symbol = args.symbol.into();

    drop(data);

    let extensions = Asset::get_extensions(&ctx.accounts.asset.data.borrow());

    if !extensions.is_empty() {
        msg!("Asset created with {:?} extension(s)", extensions);
    }

    Ok(())
}
