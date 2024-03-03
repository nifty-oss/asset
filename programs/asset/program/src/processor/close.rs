use nifty_asset_types::state::Discriminator;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{CloseAccounts, Context},
    require,
    utils::close_program_account,
};

/// Closes an uninitialized asset buffer account.
///
/// ### Accounts:
///
///   0. `[writable, signer]` buffer
///   1. `[writable]` destination
pub fn process_close(program_id: &Pubkey, ctx: Context<CloseAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.buffer.is_signer,
        ProgramError::MissingRequiredSignature,
        "asset"
    );

    require!(
        ctx.accounts.buffer.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let data = (*ctx.accounts.buffer.data).borrow();

    // make sure that the asset is uninitialized
    require!(
        data[0] == Discriminator::Uninitialized.into(),
        AssetError::AlreadyInitialized,
        "asset"
    );

    drop(data);

    // close the buffer account

    close_program_account(ctx.accounts.buffer, ctx.accounts.destination)
}
