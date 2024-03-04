use nifty_asset_types::{
    podded::ZeroCopy,
    state::{Asset, Delegate, Discriminator, NullablePubkey},
};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

use crate::{
    error::AssetError,
    instruction::{
        accounts::{ApproveAccounts, Context},
        DelegateInput,
    },
    require,
};

pub fn process_approve(
    program_id: &Pubkey,
    ctx: Context<ApproveAccounts>,
    args: DelegateInput,
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
        asset.owner == *ctx.accounts.owner.key,
        AssetError::InvalidHolder,
        "owner"
    );

    require!(
        ctx.accounts.owner.is_signer,
        ProgramError::MissingRequiredSignature,
        "owner"
    );

    // Find the roles to apply
    let roles = match args {
        DelegateInput::All => Delegate::ALL_ROLES_MASK,
        DelegateInput::Some { roles } => roles.iter().fold(0, |all, role| all | role.mask()),
    };

    // if there is a delegate set and it matches the delegate account, then we
    // only need to enable the roles; otherwise we are setting a new delegate
    // and replacing the existing one (if any)
    if let Some(delegate) = asset.delegate.value_mut() {
        if *delegate.address == *ctx.accounts.delegate.key {
            delegate.roles |= roles;
            return Ok(());
        }
    }

    let delegate = Delegate {
        address: NullablePubkey::new(*ctx.accounts.delegate.key),
        roles,
    };
    asset.delegate = delegate.into();

    Ok(())
}
