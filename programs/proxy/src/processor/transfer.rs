use nifty_asset_interface::{
    accounts::TransferAccounts,
    extensions::{Attributes, AttributesBuilder, ExtensionBuilder},
    instructions::{TransferCpiBuilder, UpdateCpiBuilder},
    state::Asset,
    types::ExtensionInput,
    ExtensionType,
};
use solana_program::{
    clock::Clock, entrypoint::ProgramResult, keccak::hashv, msg, program_error::ProgramError,
    pubkey::Pubkey, sysvar::Sysvar,
};

use crate::{
    fetch_signer,
    processor::{CONTENT_TYPE, IMAGE},
    require,
};

pub fn process_transfer<'a>(
    _program_id: &Pubkey,
    ctx: nifty_asset_interface::accounts::Context<'a, TransferAccounts<'a>>,
) -> ProgramResult {
    let data = (*ctx.accounts.asset.data).borrow();
    fetch_signer!(signer, nifty_asset_program, ctx, &data);

    // update the transfers "counter"

    let attributes = Asset::get::<Attributes>(&data).unwrap();
    let current: i64 = attributes
        .get("transfers")
        .unwrap_or("0")
        .trim()
        .parse()
        .map_err(|error| {
            msg!("[ERROR] {:?}", error);
            ProgramError::InvalidAccountData
        })?;

    drop(data);

    let data = AttributesBuilder::default()
        .add("transfers", &format!("{:>12}", current + 1))
        .data();

    UpdateCpiBuilder::new(nifty_asset_program)
        .asset(ctx.accounts.asset)
        .authority(ctx.accounts.asset)
        .extension(ExtensionInput {
            extension_type: ExtensionType::Attributes,
            length: data.len() as u32,
            data: Some(data),
        })
        .invoke_signed(&[&signer])?;

    // updates the blob (image)

    let timestamp = Clock::get()?.unix_timestamp;
    let mut buffer: [u8; 3] = [0; 3];
    buffer.copy_from_slice(
        &hashv(&[
            &ctx.accounts.asset.key.to_bytes(),
            &timestamp.to_le_bytes(),
            &current.to_le_bytes(),
        ])
        .as_ref()[..3],
    );

    let mut data = Vec::with_capacity(2000);
    data.push(CONTENT_TYPE.len() as u8);
    data.extend_from_slice(CONTENT_TYPE.as_bytes());
    data.extend_from_slice(
        IMAGE
            .replace(
                "{#RGB#}",
                &format!("{:>3},{:>3},{:>3}", buffer[0], buffer[1], buffer[2]),
            )
            .as_bytes(),
    );

    UpdateCpiBuilder::new(nifty_asset_program)
        .asset(ctx.accounts.asset)
        .authority(ctx.accounts.asset)
        .extension(ExtensionInput {
            extension_type: ExtensionType::Blob,
            length: data.len() as u32,
            data: Some(data),
        })
        .invoke_signed(&[&signer])?;

    // cpi into the Nifty Asset program to perform the transfer

    TransferCpiBuilder::new(nifty_asset_program)
        .asset(ctx.accounts.asset)
        .signer(ctx.accounts.signer)
        .recipient(ctx.accounts.recipient)
        .invoke_signed(&[&signer])
}
