use mpl_token_metadata::{
    accounts::Metadata,
    types::{Collection, TokenStandard},
};
use nifty_asset::instructions::CreateCpiBuilder;
use podded::ZeroCopy;
use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::{
    err,
    error::BridgeError,
    instruction::accounts::{Context, CreateAccounts},
    processor::SPL_TOKEN_PROGRAM_IDS,
    require,
    state::{
        bridged_asset::{self, BRIDGED_ASSET_PREFIX},
        Discriminator, State, Vault,
    },
};

pub fn process_create(program_id: &Pubkey, ctx: Context<CreateAccounts>) -> ProgramResult {
    // account validation

    require!(
        SPL_TOKEN_PROGRAM_IDS.contains(ctx.accounts.mint.owner),
        ProgramError::IllegalOwner,
        "mint"
    );

    require!(
        *ctx.accounts.metadata.owner == mpl_token_metadata::ID,
        ProgramError::IllegalOwner,
        "metadata"
    );

    // relationship validation

    let metadata = Metadata::try_from(ctx.accounts.metadata)?;

    require!(
        matches!(metadata.token_standard, Some(TokenStandard::NonFungible)),
        ProgramError::InvalidAccountData,
        "invalid token standard"
    );

    require!(
        metadata.mint == *ctx.accounts.mint.key,
        BridgeError::InvalidMint,
        "mint"
    );

    require!(
        metadata.update_authority == *ctx.accounts.update_authority.key,
        BridgeError::InvalidAuthority,
        "update_authority"
    );

    let authority_as_signer = if let Some(Collection { verified, key }) = metadata.collection {
        // if collection is verified, then we require the collection NFT
        // to be bridged as well
        if verified {
            let (derived_key, _) = bridged_asset::find_pda(&key);

            let collection_asset = if let Some(collection_asset) = ctx.accounts.collection {
                collection_asset
            } else {
                return err!(ProgramError::NotEnoughAccountKeys, "collection_asset");
            };

            require!(
                derived_key == *collection_asset.key,
                ProgramError::InvalidSeeds,
                "collection"
            );

            require!(
                collection_asset.owner == &nifty_asset::ID,
                ProgramError::IllegalOwner,
                "collection"
            );

            let collection_data = collection_asset.try_borrow_data()?;

            require!(
                !collection_data.is_empty()
                    && collection_data[0] == nifty_asset::types::Discriminator::Asset as u8,
                ProgramError::UninitializedAccount,
                "collection"
            );

            msg!("Collection found on the bridge");
        }

        !verified
    } else {
        true
    };

    // if the token does not belong to a collection or the collection is unverified,
    // then we require the update authority as a signer
    if authority_as_signer {
        require!(
            ctx.accounts.update_authority.is_signer,
            ProgramError::MissingRequiredSignature,
            "update_authority"
        );

        msg!("Collection update authority verified");
    }

    // create bridge vault account

    let mut seeds = vec![Vault::PREFIX, ctx.accounts.mint.key.as_ref()];
    let (derived_key, bump) = Pubkey::find_program_address(&seeds, program_id);

    require!(
        derived_key == *ctx.accounts.vault.key,
        ProgramError::InvalidSeeds,
        "vault"
    );

    let bump = [bump];
    seeds.push(&bump);

    if ctx.accounts.vault.data_is_empty() {
        invoke_signed(
            &system_instruction::create_account(
                ctx.accounts.payer.key,
                ctx.accounts.vault.key,
                Rent::get()?.minimum_balance(Vault::LEN),
                Vault::LEN as u64,
                program_id,
            ),
            &[ctx.accounts.payer.clone(), ctx.accounts.vault.clone()],
            &[&seeds],
        )?;
    } else {
        return err!(
            ProgramError::AccountAlreadyInitialized,
            "Vault account \"{}\" already initialized",
            ctx.accounts.vault.key
        );
    }

    msg!("Creating vault account");

    let mut data = (*ctx.accounts.vault.data).borrow_mut();
    let vault = Vault::load_mut(&mut data);

    vault.discriminator = Discriminator::Vault;
    vault.bump = bump[0];
    vault.state = State::Idle;
    vault.mint = *ctx.accounts.mint.key;

    // create the asset into the vault

    let (derived_key, bump) = bridged_asset::find_pda(ctx.accounts.mint.key);

    require!(
        derived_key == *ctx.accounts.asset.key,
        ProgramError::InvalidSeeds,
        "asset"
    );

    msg!("Creating bridged asset account");

    vault.asset_bump = bump;
    // drop data for the cpi call
    drop(data);

    CreateCpiBuilder::new(ctx.accounts.nifty_asset_program)
        .asset(ctx.accounts.asset)
        .authority(ctx.accounts.update_authority)
        .holder(ctx.accounts.vault)
        .payer(Some(ctx.accounts.payer))
        .system_program(Some(ctx.accounts.system_program))
        .name(metadata.name)
        .invoke_signed(&[&[
            BRIDGED_ASSET_PREFIX,
            ctx.accounts.mint.key.as_ref(),
            &[bump],
        ]])
}
