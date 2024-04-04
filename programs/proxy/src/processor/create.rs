use nifty_asset_interface::{
    extensions::{AttributesBuilder, ExtensionBuilder, ProxyBuilder},
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
    // proxy

    let (derived_key, bump) =
        Pubkey::find_program_address(&[ctx.accounts.stub.key.as_ref()], program_id);

    require!(
        derived_key == *ctx.accounts.asset.key,
        ProgramError::InvalidSeeds,
        "asset"
    );

    let signer = [ctx.accounts.stub.key.as_ref(), &[bump]];

    // initialize the extensions

    let data = AttributesBuilder::with_capacity(40)
        .add("transfers", &format!("{:>12}", 0))
        .data();

    let attributes = ExtensionInput {
        extension_type: ExtensionType::Attributes,
        length: data.len() as u32,
        data: Some(data),
    };

    let mut buffer = Vec::with_capacity(2000);
    buffer.push(CONTENT_TYPE.len() as u8);
    buffer.extend_from_slice(CONTENT_TYPE.as_bytes());
    buffer.extend_from_slice(IMAGE.replace("{#RGB#}", "000,000,000").as_bytes());

    let blob = ExtensionInput {
        extension_type: ExtensionType::Blob,
        length: buffer.len() as u32,
        data: Some(buffer),
    };

    let data = ProxyBuilder::with_capacity(100)
        .set(
            program_id,
            &ctx.accounts.stub.key.to_bytes(),
            bump,
            ctx.accounts.owner.key,
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
        .authority(ctx.accounts.asset, false)
        .owner(ctx.accounts.owner)
        .payer(ctx.accounts.payer)
        .system_program(ctx.accounts.system_program)
        .name(metadata.name)
        .standard(Standard::Proxied)
        .extensions(vec![attributes, blob, proxy])
        .invoke_signed(&[&signer])
}
