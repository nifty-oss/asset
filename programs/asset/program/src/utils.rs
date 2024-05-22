use nifty_asset_types::{
    constraints::Target,
    state::{Delegate, DelegateRole},
};
use nitrate::program::AccountInfo;
use solana_program::{entrypoint::ProgramResult, pubkey::Pubkey, system_program};

use crate::{error::AssetError, require};

#[inline(always)]
pub fn close_program_account(account: &AccountInfo, recipient: &AccountInfo) -> ProgramResult {
    // transfer lamports from the account to the destination account.
    let mut destination_lamports = recipient.try_borrow_mut_lamports()?;
    let mut source_lamports = account.try_borrow_mut_lamports()?;

    *destination_lamports = (*destination_lamports)
        .checked_add(*source_lamports)
        .unwrap();
    *source_lamports = 0;

    // realloc the account data size to 0 bytes and re-assign ownership of
    // the account to the system program
    account.realloc(0, false)?;
    account.assign(&system_program::ID);

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
                    *delegate.address == *target,
                    AssetError::InvalidDelegate,
                    "invalid delegate"
                );

                require!(
                    delegate.is_active(role),
                    AssetError::DelegateRoleNotActive,
                    "missing \"{:?}\" role",
                    role
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
    ( $ctx:expr, $data:expr) => {{
        // Check if royalties extension is present.
        if let Some(royalties) = Asset::get::<Royalties>($data) {
            // Check if the recipient is allowed to receive the asset.

            // Wallet-to-wallet transfers between system program accounts are exempt from the
            // royalty check so we need to exclude them.
            //
            // To determine if the transaction is a wallet-to-wallet transfer, we check:
            //
            //   1. Are we in a CPI? If so, the signer could be a ghost PDA so we
            //      cannot prove it's a wallet.
            //
            //   2. Are both the sender and the recipient system program accounts?
            let is_wallet_to_wallet = !(get_stack_height() > TRANSACTION_LEVEL_STACK_HEIGHT)
                && ($ctx.accounts.signer.owner() == &solana_program::system_program::ID)
                && ($ctx.accounts.recipient.owner() == &solana_program::system_program::ID);

            if !is_wallet_to_wallet {
                #[cfg(feature = "logging")]
                solana_program::msg!("Checking royalties constraint");

                // We pass in the `ConstraintContext` and validate the royalties constraint.
                let result = royalties.constraint.assertable.assert(&ConstraintContext {
                    asset: &$crate::utils::Account($ctx.accounts.asset),
                    authority: &$crate::utils::Account($ctx.accounts.signer),
                    recipient: Some(&$crate::utils::Account($ctx.accounts.recipient)),
                })?;

                require!(
                    result == Assertion::Pass,
                    AssetError::AssertionFailure,
                    "constraint failed"
                );
            }

            // royalties checked
            true
        } else {
            // no royalties extension
            false
        }
    }};
}

pub struct Account<'a>(pub &'a AccountInfo);

impl Target for Account<'_> {
    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.0.data_is_empty()
    }

    #[inline(always)]
    fn key(&self) -> &Pubkey {
        self.0.key()
    }

    #[inline(always)]
    fn owner(&self) -> &Pubkey {
        self.0.owner()
    }
}
