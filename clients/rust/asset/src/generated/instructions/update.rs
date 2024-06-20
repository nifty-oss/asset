//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::generated::types::ExtensionInput;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Update {
    /// Asset account
    pub asset: solana_program::pubkey::Pubkey,
    /// The authority of the asset
    pub authority: solana_program::pubkey::Pubkey,
    /// Extension (asset) buffer account
    pub buffer: Option<solana_program::pubkey::Pubkey>,
    /// The asset defining the group, if applicable
    pub group: Option<solana_program::pubkey::Pubkey>,
    /// The account paying for the storage fees
    pub payer: Option<solana_program::pubkey::Pubkey>,
    /// The system program
    pub system_program: Option<solana_program::pubkey::Pubkey>,
}

impl Update {
    pub fn instruction(
        &self,
        args: UpdateInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: UpdateInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        if let Some(buffer) = self.buffer {
            accounts.push(solana_program::instruction::AccountMeta::new(buffer, false));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(group) = self.group {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                group, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(payer, true));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(system_program) = self.system_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                system_program,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        accounts.extend_from_slice(remaining_accounts);
        let mut data = UpdateInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UpdateInstructionData {
    discriminator: u8,
}

impl UpdateInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 10 }
    }
}

impl Default for UpdateInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateInstructionArgs {
    pub name: Option<String>,
    pub mutable: Option<bool>,
    pub extension: Option<ExtensionInput>,
}

/// Instruction builder for `Update`.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` authority
///   2. `[writable, optional]` buffer
///   3. `[optional]` group
///   4. `[writable, signer, optional]` payer
///   5. `[optional]` system_program
#[derive(Clone, Debug, Default)]
pub struct UpdateBuilder {
    asset: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    buffer: Option<solana_program::pubkey::Pubkey>,
    group: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    name: Option<String>,
    mutable: Option<bool>,
    extension: Option<ExtensionInput>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UpdateBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    /// The authority of the asset
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// Extension (asset) buffer account
    #[inline(always)]
    pub fn buffer(&mut self, buffer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.buffer = buffer;
        self
    }
    /// `[optional account]`
    /// The asset defining the group, if applicable
    #[inline(always)]
    pub fn group(&mut self, group: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.group = group;
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(&mut self, payer: Option<solana_program::pubkey::Pubkey>) -> &mut Self {
        self.payer = payer;
        self
    }
    /// `[optional account]`
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: Option<solana_program::pubkey::Pubkey>,
    ) -> &mut Self {
        self.system_program = system_program;
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn mutable(&mut self, mutable: bool) -> &mut Self {
        self.mutable = Some(mutable);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn extension(&mut self, extension: ExtensionInput) -> &mut Self {
        self.extension = Some(extension);
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
        let accounts = Update {
            asset: self.asset.expect("asset is not set"),
            authority: self.authority.expect("authority is not set"),
            buffer: self.buffer,
            group: self.group,
            payer: self.payer,
            system_program: self.system_program,
        };
        let args = UpdateInstructionArgs {
            name: self.name.clone(),
            mutable: self.mutable.clone(),
            extension: self.extension.clone(),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `update` CPI accounts.
pub struct UpdateCpiAccounts<'a, 'b> {
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Extension (asset) buffer account
    pub buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The asset defining the group, if applicable
    pub group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The account paying for the storage fees
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
}

/// `update` CPI instruction.
pub struct UpdateCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the asset
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
    /// Extension (asset) buffer account
    pub buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The asset defining the group, if applicable
    pub group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The account paying for the storage fees
    pub payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The system program
    pub system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// The arguments for the instruction.
    pub __args: UpdateInstructionArgs,
}

impl<'a, 'b> UpdateCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UpdateCpiAccounts<'a, 'b>,
        args: UpdateInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            asset: accounts.asset,
            authority: accounts.authority,
            buffer: accounts.buffer,
            group: accounts.group,
            payer: accounts.payer,
            system_program: accounts.system_program,
            __args: args,
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
        let mut accounts = Vec::with_capacity(6 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        if let Some(buffer) = self.buffer {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *buffer.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(group) = self.group {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *group.key, false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(payer) = self.payer {
            accounts.push(solana_program::instruction::AccountMeta::new(
                *payer.key, true,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        if let Some(system_program) = self.system_program {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                *system_program.key,
                false,
            ));
        } else {
            accounts.push(solana_program::instruction::AccountMeta::new_readonly(
                crate::ASSET_ID,
                false,
            ));
        }
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = UpdateInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(6 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset.clone());
        account_infos.push(self.authority.clone());
        if let Some(buffer) = self.buffer {
            account_infos.push(buffer.clone());
        }
        if let Some(group) = self.group {
            account_infos.push(group.clone());
        }
        if let Some(payer) = self.payer {
            account_infos.push(payer.clone());
        }
        if let Some(system_program) = self.system_program {
            account_infos.push(system_program.clone());
        }
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

/// Instruction builder for `Update` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` asset
///   1. `[signer]` authority
///   2. `[writable, optional]` buffer
///   3. `[optional]` group
///   4. `[writable, signer, optional]` payer
///   5. `[optional]` system_program
#[derive(Clone, Debug)]
pub struct UpdateCpiBuilder<'a, 'b> {
    instruction: Box<UpdateCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UpdateCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UpdateCpiBuilderInstruction {
            __program: program,
            asset: None,
            authority: None,
            buffer: None,
            group: None,
            payer: None,
            system_program: None,
            name: None,
            mutable: None,
            extension: None,
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
    /// The authority of the asset
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// `[optional account]`
    /// Extension (asset) buffer account
    #[inline(always)]
    pub fn buffer(
        &mut self,
        buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.buffer = buffer;
        self
    }
    /// `[optional account]`
    /// The asset defining the group, if applicable
    #[inline(always)]
    pub fn group(
        &mut self,
        group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.group = group;
        self
    }
    /// `[optional account]`
    /// The account paying for the storage fees
    #[inline(always)]
    pub fn payer(
        &mut self,
        payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.payer = payer;
        self
    }
    /// `[optional account]`
    /// The system program
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    ) -> &mut Self {
        self.instruction.system_program = system_program;
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn name(&mut self, name: String) -> &mut Self {
        self.instruction.name = Some(name);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn mutable(&mut self, mutable: bool) -> &mut Self {
        self.instruction.mutable = Some(mutable);
        self
    }
    /// `[optional argument]`
    #[inline(always)]
    pub fn extension(&mut self, extension: ExtensionInput) -> &mut Self {
        self.instruction.extension = Some(extension);
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
        let args = UpdateInstructionArgs {
            name: self.instruction.name.clone(),
            mutable: self.instruction.mutable.clone(),
            extension: self.instruction.extension.clone(),
        };
        let instruction = UpdateCpi {
            __program: self.instruction.__program,

            asset: self.instruction.asset.expect("asset is not set"),

            authority: self.instruction.authority.expect("authority is not set"),

            buffer: self.instruction.buffer,

            group: self.instruction.group,

            payer: self.instruction.payer,

            system_program: self.instruction.system_program,
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UpdateCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    buffer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    name: Option<String>,
    mutable: Option<bool>,
    extension: Option<ExtensionInput>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
