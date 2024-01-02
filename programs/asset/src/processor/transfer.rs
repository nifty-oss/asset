use podded::{types::PodOption, ZeroCopy};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, TransferAccounts},
    require,
    state::{Asset, Delegate, DelegateRole, Discriminator},
};

pub fn process_transfer(_program_id: &Pubkey, ctx: Context<TransferAccounts>) -> ProgramResult {
    // Account must be a signer.
    require!(
        ctx.accounts.signer.is_signer,
        ProgramError::MissingRequiredSignature,
        "missing a signer"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    // Must be an initialized asset.
    require!(
        data[0] == Discriminator::Asset as u8,
        AssetError::Uninitialized,
        "unitialized asset"
    );

    let asset = Asset::load_mut(&mut data);

    let is_holder = asset.holder == *ctx.accounts.signer.key;

    let is_transfer_delegate = if let Some(delegate) = asset.delegate.value() {
        *delegate.address == *ctx.accounts.signer.key && delegate.is_active(DelegateRole::Transfer)
    } else {
        false
    };

    // Signing account must be holder or a transfer delegate.
    require!(
        is_holder || is_transfer_delegate,
        AssetError::InvalidTransferAuthority,
        "not a holder or transfer delegate"
    );

    // Self transfer short-circuits so as not to clear the delegate.
    if asset.holder == *ctx.accounts.recipient.key {
        return Ok(());
    }

    // Transfer the asset.
    asset.holder = *ctx.accounts.recipient.key;

    // Clear the delegate.
    asset.delegate = PodOption::new(Delegate::default());

    Ok(())
}
