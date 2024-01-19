use nifty_asset_types::state::{Asset, DelegateRole};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, system_program,
};

use crate::{error::AssetError, require};

pub fn close_program_account<'a>(
    account_info: &AccountInfo<'a>,
    funds_dest_account_info: &AccountInfo<'a>,
) -> ProgramResult {
    // Transfer lamports from the account to the destination account.
    let dest_starting_lamports = funds_dest_account_info.lamports();
    **funds_dest_account_info.lamports.borrow_mut() = dest_starting_lamports
        .checked_add(account_info.lamports())
        .unwrap();
    **account_info.lamports.borrow_mut() = 0;

    // Realloc the account data size to 0 bytes and re-assign ownership of
    // the account to the system program
    account_info.realloc(0, false)?;
    account_info.assign(&system_program::ID);

    Ok(())
}

#[inline(always)]
pub fn assert_delegate(asset: &Asset, target: &Pubkey, role: DelegateRole) -> ProgramResult {
    let delegate = asset.delegate.value().ok_or(AssetError::DelegateNotFound)?;

    require!(
        delegate.is_active(role),
        AssetError::DelegateRoleNotActive,
        "missing \"{:?}\" role",
        role
    );

    require!(
        *delegate.address == *target,
        AssetError::InvalidDelegate,
        "invalid delegate"
    );

    Ok(())
}
