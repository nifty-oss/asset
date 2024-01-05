use podded::ZeroCopy;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{BurnAccounts, Context},
    require,
    state::{Asset, DelegateRole, Discriminator},
    utils::{assert_delegate, close_program_account},
};

pub fn process_burn(program_id: &Pubkey, ctx: Context<BurnAccounts>) -> ProgramResult {
    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    require!(
        ctx.accounts.signer.is_signer,
        ProgramError::MissingRequiredSignature,
        "missing required signer"
    );

    let data = ctx.accounts.asset.try_borrow_data()?;

    // Must be an initialized asset.
    require!(
        data[0] == Discriminator::Asset as u8,
        AssetError::Uninitialized,
        "unitialized asset"
    );

    let asset = Asset::load(&data);

    // Validate the signer is the holder or the burn delegate.
    let is_holder = asset.holder == *ctx.accounts.signer.key;
    let is_delegate = assert_delegate(asset, ctx.accounts.signer.key, DelegateRole::Burn).is_ok();

    require!(
        is_holder || is_delegate,
        AssetError::InvalidBurnAuthority,
        "not an owner or burn delegate"
    );

    let recipient = ctx.accounts.recipient.unwrap_or(ctx.accounts.signer);

    // Free up asset account reference.
    drop(data);

    close_program_account(ctx.accounts.asset, recipient)
}
