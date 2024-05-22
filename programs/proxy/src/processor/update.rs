use nifty_asset_interface::{
    accounts::UpdateAccounts, fetch_proxy_data, instructions::UpdateCpiBuilder, UpdateInput,
};
use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::require;

pub fn process_update<'a>(
    _program_id: &Pubkey,
    ctx: nifty_asset_interface::accounts::Context<'a, UpdateAccounts<'a>>,
    input: UpdateInput,
) -> ProgramResult {
    // account validation happends on the CPI, we only need to make sure we
    // have the authority to perform the update

    let data = (*ctx.accounts.asset.data).borrow();
    fetch_proxy_data!(
        &data,
        signer,
        authority,
        nifty_asset_program,
        ctx.remaining_accounts
    );

    require!(
        ctx.accounts.authority.is_signer,
        ProgramError::MissingRequiredSignature,
        "authority"
    );

    // we need to validate the authority against the "proxy" authority
    // since the asset itself is the authority for the update
    if let Some(authority) = authority {
        require!(
            *ctx.accounts.authority.key == *authority,
            ProgramError::InvalidArgument,
            "authority mismatch"
        );
    } else {
        msg!("[ERROR] Authority not found");
        return Err(ProgramError::InvalidArgument);
    }

    drop(data);

    // cpi into the Nifty Asset program to perform the update
    // (ignores any request to update extension update)

    let mut builder = UpdateCpiBuilder::new(nifty_asset_program);

    builder
        .asset(ctx.accounts.asset)
        .authority(ctx.accounts.asset);

    if let Some(name) = input.name {
        builder.name(name);
    }

    if let Some(mutable) = input.mutable {
        builder.mutable(mutable);
    }

    if let Some(extension) = input.extension {
        msg!(
            "Ignoring [{:?}] extension update request",
            extension.extension_type
        );
    }

    builder.invoke_signed(&[&signer])
}
