//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Verify {
    /// Asset account
    pub asset: solana_program::pubkey::Pubkey,
    /// Creator account to verify
    pub creator: solana_program::pubkey::Pubkey,
}

impl Verify {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.creator,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = VerifyInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::INTERFACE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct VerifyInstructionData {
    discriminator: u8,
}

impl VerifyInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 11 }
    }
}

impl Default for VerifyInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `Verify`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[signer]` creator
#[derive(Clone, Debug, Default)]
pub struct VerifyBuilder {
    asset: Option<solana_program::pubkey::Pubkey>,
    creator: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl VerifyBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    /// Creator account to verify
    #[inline(always)]
    pub fn creator(&mut self, creator: solana_program::pubkey::Pubkey) -> &mut Self {
        self.creator = Some(creator);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Verify {
            asset: self.asset.expect("asset is not set"),
            creator: self.creator.expect("creator is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `verify` CPI accounts.
pub struct VerifyCpiAccounts<'a, 'b> {
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Creator account to verify
    pub creator: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `verify` CPI instruction.
pub struct VerifyCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Creator account to verify
    pub creator: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> VerifyCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: VerifyCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            asset: accounts.asset,
            creator: accounts.creator,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(2 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.creator.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = VerifyInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::INTERFACE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(2 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset.clone());
        account_infos.push(self.creator.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Verify` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[signer]` creator
#[derive(Clone, Debug)]
pub struct VerifyCpiBuilder<'a, 'b> {
    instruction: Box<VerifyCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> VerifyCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(VerifyCpiBuilderInstruction {
            __program: program,
            asset: None,
            creator: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asset = Some(asset);
        self
    }
    /// Creator account to verify
    #[inline(always)]
    pub fn creator(
        &mut self,
        creator: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.creator = Some(creator);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = VerifyCpi {
            __program: self.instruction.__program,

            asset: self.instruction.asset.expect("asset is not set"),

            creator: self.instruction.creator.expect("creator is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct VerifyCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    creator: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
