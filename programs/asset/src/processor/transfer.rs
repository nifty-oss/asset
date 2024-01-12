use podded::{pod::PodOption, ZeroCopy};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, TransferAccounts},
    require,
    state::{Asset, Delegate, DelegateRole, Discriminator},
    utils::assert_delegate,
};

pub fn process_transfer(program_id: &Pubkey, ctx: Context<TransferAccounts>) -> ProgramResult {
    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

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

    let is_delegate =
        assert_delegate(asset, ctx.accounts.signer.key, DelegateRole::Transfer).is_ok();

    // Signing account must be holder or a transfer delegate.
    require!(
        is_holder || is_delegate,
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
