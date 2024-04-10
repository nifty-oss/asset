use nifty_asset_interface::{
    extensions::{AttributesBuilder, BlobBuilder, ExtensionBuilder, ProxyBuilder},
    instructions::CreateCpiBuilder,
    types::ExtensionInput,
    ExtensionType, Standard,
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    instruction::{
        accounts::{Context, CreateAccounts},
        Metadata,
    },
    processor::{CONTENT_TYPE, IMAGE},
    require,
};

pub fn process_create(
    program_id: &Pubkey,
    ctx: Context<CreateAccounts>,
    metadata: Metadata,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.authority.is_signer,
        ProgramError::MissingRequiredSignature,
        "authority"
    );

    require!(
        ctx.accounts.stub.is_signer,
        ProgramError::MissingRequiredSignature,
        "stub"
    );

    let (derived_key, bump) =
        Pubkey::find_program_address(&[ctx.accounts.stub.key.as_ref()], program_id);

    require!(
        derived_key == *ctx.accounts.asset.key,
        ProgramError::InvalidSeeds,
        "asset"
    );

    let signer = [ctx.accounts.stub.key.as_ref(), &[bump]];

    // initialize the extensions

    let data = AttributesBuilder::with_capacity(25)
        .add("transfers", &format!("{:>12}", 0))
        .data();

    let attributes = ExtensionInput {
        extension_type: ExtensionType::Attributes,
        length: data.len() as u32,
        data: Some(data),
    };

    let data = BlobBuilder::with_capacity(2000)
        .set_data(
            CONTENT_TYPE,
            IMAGE.replace("{#RGB#}", "000,000,000").as_bytes(),
        )
        .data();

    let blob = ExtensionInput {
        extension_type: ExtensionType::Blob,
        length: data.len() as u32,
        data: Some(data),
    };

    let data = ProxyBuilder::with_capacity(100)
        .set(
            program_id,
            &ctx.accounts.stub.key.to_bytes(),
            bump,
            // "proxy" authority
            Some(ctx.accounts.authority.key),
        )
        .data();

    let proxy = ExtensionInput {
        extension_type: ExtensionType::Proxy,
        length: data.len() as u32,
        data: Some(data),
    };

    // creates the proxied asset

    CreateCpiBuilder::new(ctx.accounts.nifty_asset_program)
        .asset(ctx.accounts.asset)
        // keeps the authority so we can update the asset
        .authority(ctx.accounts.asset, true)
        .owner(ctx.accounts.owner)
        .payer(ctx.accounts.payer)
        .system_program(ctx.accounts.system_program)
        .name(metadata.name)
        .standard(Standard::Proxied)
        .extensions(vec![attributes, blob, proxy])
        .invoke_signed(&[&signer])
}
