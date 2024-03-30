use mpl_token_metadata::{
    accounts::Metadata, instructions::TransferV1CpiBuilder, types::TokenStandard,
};
use nifty_asset::instructions::TransferCpiBuilder;
use podded::ZeroCopy;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    error::BridgeError,
    instruction::accounts::{BridgeAccounts, Context},
    processor::SPL_TOKEN_PROGRAM_IDS,
    require,
    state::{bridged_asset, Discriminator, State, Vault},
};

/// Struct to hold the information for the token and asset transfer CPI calls.
struct TransferAccounts<'a> {
    /// Authority of the transfer.
    authority: &'a AccountInfo<'a>,
    /// The account to transfer from.
    source: &'a AccountInfo<'a>,
    /// Token account to transfer from.
    source_token: &'a AccountInfo<'a>,
    /// Token record account to transfer from.
    source_token_record: Option<&'a AccountInfo<'a>>,
    /// The account receiving the tokens.
    destination: &'a AccountInfo<'a>,
    /// Token account to transfer to.
    destination_token: &'a AccountInfo<'a>,
    /// Token record account receiving the tokens.
    destination_token_record: Option<&'a AccountInfo<'a>>,
}

pub fn process_bridge(program_id: &Pubkey, ctx: Context<BridgeAccounts>) -> ProgramResult {
    // account validation (most accounts are validated on token metadata cpi)
    // we only need to make sure that we got the correct asset and vault accounts
    // for the mint

    require!(
        ctx.accounts.owner.is_signer,
        ProgramError::MissingRequiredSignature,
        "owner"
    );

    require!(
        SPL_TOKEN_PROGRAM_IDS.contains(ctx.accounts.mint.owner),
        ProgramError::IllegalOwner,
        "mint"
    );

    require!(
        *ctx.accounts.asset.owner == nifty_asset::ID,
        ProgramError::IllegalOwner,
        "asset"
    );

    require!(
        ctx.accounts.vault.owner == program_id,
        ProgramError::IllegalOwner,
        "vault"
    );

    // derivation validation

    let mut data = (*ctx.accounts.vault.data).borrow_mut();
    // vault account must be initialized
    require!(
        !data.is_empty() && data[0] == Discriminator::Vault as u8,
        ProgramError::UninitializedAccount,
        "vault"
    );

    let vault = Vault::load_mut(&mut data);

    let seeds = [Vault::PREFIX, ctx.accounts.mint.key.as_ref(), &[vault.bump]];
    let derived_key = Pubkey::create_program_address(&seeds, program_id)?;

    require!(
        derived_key == *ctx.accounts.vault.key,
        ProgramError::InvalidSeeds,
        "bridge"
    );

    require!(
        vault.mint == *ctx.accounts.mint.key,
        BridgeError::InvalidMint,
        "mint"
    );

    let derived_key = bridged_asset::create_pda(*ctx.accounts.mint.key, vault.asset_bump)?;

    require!(
        derived_key == *ctx.accounts.asset.key,
        ProgramError::InvalidSeeds,
        "asset"
    );

    // relationship validation

    let metadata = Metadata::try_from(ctx.accounts.metadata)?;

    require!(
        matches!(
            metadata.token_standard,
            Some(TokenStandard::NonFungible) | Some(TokenStandard::ProgrammableNonFungible)
        ),
        ProgramError::InvalidAccountData,
        "metadata"
    );

    require!(
        metadata.mint == *ctx.accounts.mint.key,
        ProgramError::InvalidAccountData,
        "mint"
    );

    // bridges token <-> asset

    let (transfer_accounts, state) = match vault.state {
        State::Idle => (
            TransferAccounts {
                authority: ctx.accounts.owner,
                source: ctx.accounts.owner,
                source_token: ctx.accounts.token,
                source_token_record: ctx.accounts.token_record,
                destination: ctx.accounts.vault,
                destination_token: ctx.accounts.vault_token,
                destination_token_record: ctx.accounts.vault_token_record,
            },
            State::Active,
        ),
        State::Active => (
            TransferAccounts {
                authority: ctx.accounts.vault,
                source: ctx.accounts.vault,
                source_token: ctx.accounts.vault_token,
                source_token_record: ctx.accounts.vault_token_record,
                destination: ctx.accounts.owner,
                destination_token: ctx.accounts.token,
                destination_token_record: ctx.accounts.token_record,
            },
            State::Idle,
        ),
    };

    vault.state = state;
    // drop the data reference for cpi call
    drop(data);

    let mut token_transfer_cpi = TransferV1CpiBuilder::new(ctx.accounts.token_metadata_program);
    token_transfer_cpi
        .token(transfer_accounts.source_token)
        .token_owner(transfer_accounts.source)
        .destination_token(transfer_accounts.destination_token)
        .destination_owner(transfer_accounts.destination)
        .mint(ctx.accounts.mint)
        .metadata(ctx.accounts.metadata)
        .edition(Some(ctx.accounts.master_edition))
        .token_record(transfer_accounts.source_token_record)
        .destination_token_record(transfer_accounts.destination_token_record)
        .authority(transfer_accounts.authority)
        .payer(ctx.accounts.payer)
        .system_program(ctx.accounts.system_program)
        .sysvar_instructions(ctx.accounts.sysvar_instructions)
        .spl_token_program(ctx.accounts.spl_token_program)
        .spl_ata_program(ctx.accounts.spl_ata_program)
        .authorization_rules_program(ctx.accounts.authorization_rules_program)
        .authorization_rules(ctx.accounts.authorization_rules)
        .amount(1);

    let mut asset_transfer_cpi = TransferCpiBuilder::new(ctx.accounts.nifty_asset_program);
    asset_transfer_cpi
        .asset(ctx.accounts.asset)
        .recipient(transfer_accounts.source)
        .group(ctx.accounts.group_asset)
        .signer(transfer_accounts.destination);

    // this is the state that the vault will be in after the cpi call
    match state {
        State::Active => {
            msg!("Bridging token to asset");
            token_transfer_cpi.invoke()?;
            asset_transfer_cpi.invoke_signed(&[&seeds])
        }
        State::Idle => {
            msg!("Bridging asset to token");
            token_transfer_cpi.invoke_signed(&[&seeds])?;
            asset_transfer_cpi.invoke()
        }
    }
}
