use crate::instructions::AllocateCpiAccounts;

impl<'a, 'b> AllocateCpiAccounts<'a, 'b> {
    /// Invokes the `Allocate` instruction.
    ///
    /// This invocation expects the instruction data to be directly provided, which
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
    /// This invocation expects the instruction data to be directly provided, which
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
///      correct capacity. This is useful when you want to write the data (bytes) iteratively
///      without having to allocate a new `Vec` for it. The length of the data can be updated
///      later using the `update_data_length!` macro.
///
/// The macro will return a `Vec<u8>` containing the instruction data.
///
/// # Arguments
///
/// 1. `extension_type` - the type ([`ExtensionType`]) of the extension.
/// 2. `length` - expression representing the length of the extension data.
/// 3. `data` - (optional) expression representing the extension data as a byte slice.
#[macro_export]
macro_rules! allocate_instruction_data {
    ( $extension_type:expr, $length:expr, $data:expr ) => {{
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

/// Updates the length of the extension data in the `Allocate` instruction data.
#[macro_export]
macro_rules! allocate_update_data_length {
    ( $length:expr, $data:expr ) => {{
        let length_index = std::mem::size_of::<u8>() // discriminator
            + std::mem::size_of::<u8>();             // extension type

        let data_length_index = length_index
            + std::mem::size_of::<u32>()             // length
            + std::mem::size_of::<u8>();             // option

        $data[length_index..length_index + std::mem::size_of::<u32>()].copy_from_slice(&u32::to_le_bytes($length as u32));
        $data[data_length_index..data_length_index + std::mem::size_of::<u32>()].copy_from_slice(&u32::to_le_bytes($length as u32));
    }};
}

/// Convenience macro to invoke `Allocate` and `Write` instructions.
///
/// The macro will generate the sufficient invokes to allocated and write all the extension data.
///
/// # Arguments
///
/// 1. `program` - expression represeting the reference to the nifty asset program account info
/// 2. `asset` - expression representing the reference to the asset account info
/// 3. `payer` - expression representing the reference to the payer account info
/// 4. `system_program` - expression representing the reference to the system program account info
/// 5. `extension_type` - the type ([`ExtensionType`]) of the extension.
/// 6. `data` - expression representing the extension data as a byte slice.
/// 7. `signers_seeds` - (optional) expression representing the reference to the signers seeds
#[macro_export]
macro_rules! allocate_and_write {
    ( $program:expr, $asset:expr, $payer:expr, $system_program:expr, $extension_type:expr, $data:expr, $signers_seeds:expr ) => {{
        const CPI_LIMIT: usize = 1280;
        const ACCOUNT_META_SIZE: usize = 34; // 32 bytes for pubkey + 1 byte for is_signer + 1 byte for is_writable

        let total_data_len = $data.len();
        // (1) discriminator
        // (1) extension type
        // (4) length
        // (1) option
        // (4) data length
        // total = 11
        const ALLOCATE_HEADER: usize = 11;

        let accounts = vec![
            solana_program::instruction::AccountMeta::new(*$asset.key, true),
            solana_program::instruction::AccountMeta::new(*$payer.key, true),
            solana_program::instruction::AccountMeta::new_readonly(*$system_program.key, false),
        ];
        let account_metas_size = accounts.len() * ACCOUNT_META_SIZE; 

        let data_len = std::cmp::min(total_data_len, CPI_LIMIT - ALLOCATE_HEADER - account_metas_size);


        let account_infos = vec![
            $program.clone(),
            $asset.clone(),
            $payer.clone(),
            $system_program.clone(),
        ];

        let mut instruction_data = Vec::with_capacity(CPI_LIMIT);
        instruction_data.push(4); // allocate discriminator
        instruction_data.push($extension_type as u8);
        instruction_data.extend_from_slice(&u32::to_le_bytes(total_data_len as u32));
        instruction_data.push(1);
        instruction_data.extend_from_slice(&u32::to_le_bytes(data_len as u32));
        instruction_data.extend_from_slice(&$data[..data_len]);

        let mut instruction = solana_program::instruction::Instruction {
            program_id: nifty_asset::ID,
            accounts,
            data: instruction_data,
        };

        solana_program::program::invoke_signed(&instruction, &account_infos, $signers_seeds)?;

        let mut total = data_len;

        while total < total_data_len {
            instruction.data.clear();
            let offset = total;
            // (1) discriminator
            // (1) overwrite
            // total = 2
            const WRITE_HEADER: usize = 2;
            let data_len = std::cmp::min(total_data_len - offset, CPI_LIMIT - WRITE_HEADER - account_metas_size);

            instruction.data.push(12); // write discriminator
            instruction.data.push(0); // overwrite (false)
            instruction
                .data
                .extend_from_slice(&u32::to_le_bytes(data_len as u32));
            instruction
                .data
                .extend_from_slice(&$data[offset..offset + data_len]);

            solana_program::program::invoke_signed(&instruction, &account_infos, $signers_seeds)?;

            total += data_len;
        }
    }};
    ( $program:expr, $asset:expr, $payer:expr, $system_program:expr, $extension_type:expr, $data:expr ) => {{
        allocate_and_write!(
            $program,
            $asset,
            $payer,
            $system_program,
            $extension_type,
            $data,
            &[]
        );
    }};
}
