use podded::ZeroCopy;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    instruction::accounts::{Context, UnlockAccounts},
    require,
    state::{Asset, DelegateRole, Discriminator, State},
    utils::assert_delegate,
};

pub fn process_unlock(program_id: &Pubkey, ctx: Context<UnlockAccounts>) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.delegate.is_signer,
        ProgramError::MissingRequiredSignature,
        "delegate"
    );

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data[0] == Discriminator::Asset as u8,
        ProgramError::UninitializedAccount,
        "asset"
    );

    // unlocks the asset

    let asset = Asset::load_mut(&mut data);

    assert_delegate(asset, ctx.accounts.delegate.key, DelegateRole::Lock)?;

    asset.state = State::Unlocked;

    Ok(())
}
