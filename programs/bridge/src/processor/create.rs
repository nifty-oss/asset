use mpl_token_metadata::{
    accounts::Metadata,
    types::{Collection, TokenStandard},
};
use nifty_asset::{
    extensions::{ExtensionBuilder, GroupBuilder, MetadataBuilder, RoyaltiesBuilder},
    instructions::{AllocateCpiBuilder, CreateCpiBuilder},
    types::{ExtensionInput, ExtensionType},
};
use nifty_asset_types::constraints::{Account, NotBuilder, PubkeyMatchBuilder};
use podded::ZeroCopy;
use solana_program::{
    entrypoint::ProgramResult, msg, program::invoke_signed, program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar,
};

use crate::{
    err,
    error::BridgeError,
    instruction::{
        accounts::{Context, CreateAccounts},
        CreateArgs,
    },
    processor::SPL_TOKEN_PROGRAM_IDS,
    require,
    state::{
        bridged_asset::{self, BRIDGED_ASSET_PREFIX},
        Discriminator, State, Vault,
    },
};

pub fn process_create(
    program_id: &Pubkey,
    ctx: Context<CreateAccounts>,
    args: CreateArgs,
) -> ProgramResult {
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
        matches!(
            metadata.token_standard,
            Some(TokenStandard::NonFungible) | Some(TokenStandard::ProgrammableNonFungible)
        ),
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
                    && collection_data[0] == nifty_asset::state::Discriminator::Asset.into(),
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

    let signer_seeds = [
        BRIDGED_ASSET_PREFIX,
        ctx.accounts.mint.key.as_ref(),
        &[bump],
    ];

    let mut extension = MetadataBuilder::default();
    extension.set(Some(&metadata.symbol), None, Some(&metadata.uri));
    let data = extension.build();

    AllocateCpiBuilder::new(ctx.accounts.nifty_asset_program)
        .asset(ctx.accounts.asset)
        .payer(Some(ctx.accounts.payer))
        .system_program(Some(ctx.accounts.system_program))
        .extension(ExtensionInput {
            extension_type: ExtensionType::Metadata,
            length: data.len() as u32,
            data: Some(data),
        })
        .invoke_signed(&[&signer_seeds])?;

    // if this is a collection NFT, we add the group extension
    if args.is_collection || metadata.collection_details.is_some() {
        let mut extension = GroupBuilder::default();
        let data = extension.build();

        AllocateCpiBuilder::new(ctx.accounts.nifty_asset_program)
            .asset(ctx.accounts.asset)
            .payer(Some(ctx.accounts.payer))
            .system_program(Some(ctx.accounts.system_program))
            .extension(ExtensionInput {
                extension_type: ExtensionType::Grouping,
                length: data.len() as u32,
                data: Some(data),
            })
            .invoke_signed(&[&signer_seeds])?;

        // for pNFTs we also add a default Royalties extension
        if metadata.token_standard == Some(TokenStandard::ProgrammableNonFungible) {
            // we set a not pubkey match with the system pubkey as the default, as this should be a
            // pass-all rule
            let mut pubkey_match_builder = PubkeyMatchBuilder::default();
            pubkey_match_builder.set(Account::Asset, &[Pubkey::default()]);

            let mut not_builder = NotBuilder::default();
            not_builder.set(&mut pubkey_match_builder);

            let mut extension = RoyaltiesBuilder::default();
            extension.set(metadata.seller_fee_basis_points as u64, &mut not_builder);
            let royalties_data = extension.data();

            AllocateCpiBuilder::new(ctx.accounts.nifty_asset_program)
                .asset(ctx.accounts.asset)
                .payer(Some(ctx.accounts.payer))
                .system_program(Some(ctx.accounts.system_program))
                .extension(ExtensionInput {
                    extension_type: ExtensionType::Royalties,
                    length: royalties_data.len() as u32,
                    data: Some(royalties_data),
                })
                .invoke_signed(&[&signer_seeds])?;
        }
    }

    CreateCpiBuilder::new(ctx.accounts.nifty_asset_program)
        .asset(ctx.accounts.asset)
        .authority(ctx.accounts.update_authority)
        .owner(ctx.accounts.vault)
        .payer(Some(ctx.accounts.payer))
        .system_program(Some(ctx.accounts.system_program))
        .name(metadata.name)
        .invoke_signed(&[&signer_seeds])?;

    Ok(())
}
