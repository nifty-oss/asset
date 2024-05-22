use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::Interface;

/// Fetches the data for a proxy asset.
///
/// This macro is used to fetch the data from the proxy extension of an asset.
///
/// # Arguments
///
/// 1. `data` - expression representing the account data.
/// 2. `signer` - identifier to store the signer seeds.
/// 3. `authority` - identifier to store the authority pubkey.
/// 4. `program` - (optional) identifier to store the program pubkey.
/// 5. `remaining` - (optional) expression representing the remaining accounts array.
#[macro_export]
macro_rules! fetch_proxy_data {
    ( $data:expr, $signer:ident, $authority:ident ) => {
        // proxy
        let __proxy = $crate::state::Asset::get::<$crate::extensions::Proxy>($data)
            .ok_or(solana_program::program_error::ProgramError::InvalidAccountData)?;

        let __seeds = *__proxy.seeds;
        let $signer = [__seeds.as_ref(), &[*__proxy.bump]];

        let $authority = if let Some(authority) = __proxy.authority.value() {
            Some(*authority)
        } else {
            None
        };
    };

    ( $data:expr, $signer:ident, $authority:ident, $program:ident, $remaining:expr ) => {
        fetch_proxy_data!($data, $signer, $authority);

        let $program = $remaining
            .iter()
            .find(|account| {
                account.key
                    == &solana_program::pubkey!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73")
            })
            .ok_or(solana_program::program_error::ProgramError::NotEnoughAccountKeys)?;
    };
}

/// Logs the name of the instruction.
///
/// The macro use the first byte of the instruction data to determine
/// the name of the instruction.
macro_rules! log_instruction_name {
    ( $instruction_data:expr ) => {{
        if $instruction_data.is_empty() {
            return Err(solana_program::program_error::ProgramError::InvalidInstructionData);
        }

        let name = match $instruction_data[0] {
            0 => "Close",
            1 => "Burn",
            2 => "Create",
            3 => "Approve",
            4 => "Allocate",
            5 => "Lock",
            6 => "Revoke",
            7 => "Transfer",
            8 => "Unlock",
            9 => "Unverify",
            10 => "Update",
            11 => "Verify",
            12 => "Write",
            13 => "Group",
            14 => "Ungroup",
            15 => "Handover",
            16 => "Remove",
            _ => return Err(solana_program::program_error::ProgramError::InvalidInstructionData),
        };

        solana_program::msg!("Interface: {}", name);
    }};
}

impl Interface {
    /// Processes the instruction for the asset program.
    pub fn process_instruction<'a>(
        _program_id: &'a Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        log_instruction_name!(instruction_data);

        let asset = accounts.first().ok_or(ProgramError::NotEnoughAccountKeys)?;
        let data = (*asset.data).borrow();
        fetch_proxy_data!(&data, signer, _authority);
        // drop the data before invoking the CPI
        drop(data);

        let mut account_metas = Vec::with_capacity(accounts.len());
        accounts.iter().for_each(|account| {
            account_metas.push(AccountMeta {
                pubkey: *account.key,
                is_signer: account.is_signer,
                is_writable: account.is_writable,
            });
        });
        // the asset account must be a signer
        account_metas.first_mut().unwrap().is_signer = true;

        invoke_signed(
            &Instruction {
                accounts: account_metas,
                data: instruction_data.to_vec(),
                program_id: solana_program::pubkey!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73"),
            },
            accounts,
            &[&signer],
        )
    }
}
