use podded::ZeroCopy;
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::accounts::{Context, DelegateAccounts},
    require,
    state::{Asset, Delegate, DelegateRole, Discriminator, NullablePubkey},
};

pub fn process_delegate(
    program_id: &Pubkey,
    ctx: Context<DelegateAccounts>,
    roles: Vec<DelegateRole>,
) -> ProgramResult {
    // account validation

    require!(
        ctx.accounts.asset.owner == program_id,
        ProgramError::IllegalOwner,
        "asset"
    );

    let mut data = (*ctx.accounts.asset.data).borrow_mut();

    require!(
        data[0] == Discriminator::Asset.into(),
        ProgramError::UninitializedAccount,
        "asset"
    );

    let asset = Asset::load_mut(&mut data);

    require!(
        asset.holder == *ctx.accounts.holder.key,
        AssetError::InvalidHolder,
        "holder"
    );

    require!(
        ctx.accounts.holder.is_signer,
        ProgramError::MissingRequiredSignature,
        "holder"
    );

    // if there is a delegate set and it matches the delegate account, then we
    // only need to enable the roles; otherwise we are setting a new delegate
    // and replacing the existing one (if any)

    if let Some(delegate) = asset.delegate.value_mut() {
        if *delegate.address == *ctx.accounts.delegate.key {
            roles.iter().for_each(|role| delegate.enable(*role));
            return Ok(());
        }
    }

    let delegate = Delegate {
        address: NullablePubkey::new(*ctx.accounts.delegate.key),
        roles: roles.iter().fold(0, |all, role| all | role.mask()),
    };
    asset.delegate = delegate.into();

    Ok(())
}
