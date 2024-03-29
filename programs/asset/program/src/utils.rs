use nifty_asset_types::state::{Delegate, DelegateRole};
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

#[allow(clippy::manual_try_fold)]
#[inline(always)]
pub fn assert_delegate(
    delegates: &[Option<&Delegate>],
    target: &Pubkey,
    role: DelegateRole,
) -> ProgramResult {
    delegates.iter().fold(
        Err(AssetError::DelegateNotFound.into()),
        |result, delegate| {
            if let Some(delegate) = delegate {
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
            } else {
                result
            }
        },
    )
}

#[macro_export]
macro_rules! process_royalties {
    ( $ctx:expr, $data:expr) => {
        {
            // Check if royalties extension is present.
            if let Some(royalties) = Asset::get::<Royalties>($data) {
                // Check if the recipient is allowed to receive the asset.

                // Wallet-to-wallet transfers between system program accounts are exempt from the royalty check
                // so we need to exclude them.

                // Are we in a CPI? If so, the signer could be a ghost PDA so we cannot prove it's a wallet.
                let is_cpi = get_stack_height() > TRANSACTION_LEVEL_STACK_HEIGHT;

                // Are both the sender and the recipient system program accounts?
                let sender_is_wallet =
                    $ctx.accounts.signer.owner == &solana_program::system_program::id();
                let recipient_is_wallet =
                    $ctx.accounts.recipient.owner == &solana_program::system_program::id();

                let is_wallet_to_wallet = !is_cpi && sender_is_wallet && recipient_is_wallet;

                if !is_wallet_to_wallet {
                    msg!("Checking royalties constraint");
                    // We pass in the Constraint context and validate the royalties constraint.
                    let result = royalties.constraint.assertable.assert(&ConstraintContext {
                        asset: $ctx.accounts.asset,
                        authority: $ctx.accounts.signer,
                        recipient: Some($ctx.accounts.recipient),
                    })?;
                    require!(
                        result == Assertion::Pass,
                        AssetError::AssertionFailure,
                        "constraint failed"
                    );
                }

                true // royalties checked
            } else {
                false // no royalties extension
            }
        }
    }
}
