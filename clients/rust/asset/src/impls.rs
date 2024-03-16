use crate::instructions::AllocateCpiAccounts;

impl<'a, 'b> AllocateCpiAccounts<'a, 'b> {
    /// Invokes the `Allocate` instruction.
    ///
    /// This invokation expects the instruction data to be direcrlt provided, which
    /// will save CU compared to a Borsh equivalent. In most cases, the instruction
    /// data will be generated using `allocate_instruction_data!` macro.
    #[inline(always)]
    pub fn invoke(
        &self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
        instruction_data: Vec<u8>,
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(program, instruction_data, &[])
    }

    /// Invokes the `Allocate` instruction with the specified signers.
    ///
    /// This invokation expects the instruction data to be direcrlt provided, which
    /// will save CU compared to a Borsh equivalent. In most cases, the instruction
    /// data will be generated using `allocate_instruction_data!` macro.
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        program: &'b solana_program::account_info::AccountInfo<'a>,
        instruction_data: Vec<u8>,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let accounts = vec![
            solana_program::instruction::AccountMeta::new(*self.asset.key, true),
            if let Some(payer) = self.payer {
                solana_program::instruction::AccountMeta::new(*payer.key, true)
            } else {
                solana_program::instruction::AccountMeta::new_readonly(crate::ASSET_ID, false)
            },
            if let Some(system_program) = self.system_program {
                solana_program::instruction::AccountMeta::new_readonly(*system_program.key, false)
            } else {
                solana_program::instruction::AccountMeta::new_readonly(crate::ASSET_ID, false)
            },
        ];

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::ASSET_ID,
            accounts,
            data: instruction_data,
        };
        let mut account_infos = Vec::with_capacity(4);
        account_infos.push(program.clone());
        account_infos.push(self.asset.clone());
        if let Some(payer) = self.payer {
            account_infos.push(payer.clone());
        }
        if let Some(system_program) = self.system_program {
            account_infos.push(system_program.clone());
        }

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Convenience macro to create the instruction data for the `Allocate` instruction.
///
/// There are two forms of this macro and they vary based on how the extension data
/// is provided:
///
///   1. You can provide the data directly as a byte slice;
///   2. You can provide the length of the data and the instruction data will have the
///     correct capacity. This is useful when you want to write the data (bytes) iteratively
///     without having to allocate a new `Vec` for it.
///
/// # Arguments
///
/// 1. `extension_type` - the type ([`ExtensionType`]) of the extension.
/// 2. `length` - expression representing the length of the extension data.
/// 3. `data` - (optional) the extension data as a byte slice.
#[macro_export]
macro_rules! allocate_instruction_data {
    ( $extension_type:expr, $length:expr, $data:tt ) => {{
        let discriminator: u8 = 4;

        let mut size = std::mem::size_of::<u8>() // discriminator
            + std::mem::size_of::<u8>()          // extension type
            + std::mem::size_of::<u32>()         // length
            + std::mem::size_of::<u8>()          // option
            + std::mem::size_of::<u32>()         // data length
            + $data.len();                       // data

        let mut instruction_data = Vec::with_capacity(size);
        instruction_data.push(discriminator);
        instruction_data.push($extension_type as u8);
        instruction_data.extend_from_slice(&u32::to_le_bytes($length as u32));
        instruction_data.push(1);
        instruction_data.extend_from_slice(&u32::to_le_bytes($data.len() as u32));
        instruction_data.extend_from_slice($data);

        instruction_data
    }};

    ( $extension_type:expr, $length:expr ) => {{
        let discriminator: u8 = 4;

        let mut size = std::mem::size_of::<u8>() // discriminator
            + std::mem::size_of::<u8>()          // extension type
            + std::mem::size_of::<u32>()         // length
            + std::mem::size_of::<u8>()          // option
            + std::mem::size_of::<u32>()         // data length
            + $length as usize;                  // allocated data space

        let mut instruction_data = Vec::with_capacity(size);
        instruction_data.push(discriminator);
        instruction_data.push($extension_type as u8);
        instruction_data.extend_from_slice(&u32::to_le_bytes($length as u32));
        instruction_data.push(1);
        instruction_data.extend_from_slice(&u32::to_le_bytes($length as u32));

        instruction_data
    }};
}
