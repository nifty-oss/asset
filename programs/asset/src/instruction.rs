use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

use crate::extensions::ExtensionType;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum DASInstruction {
    /// Create a new asset.
    #[account(0, writable, name="asset", desc = "Asset account (pda of `['asset', mold pubkey]`)")]
    #[account(1, signer, name="mold", desc = "Address to derive the PDA from")]
    #[account(2, signer, name="authority", desc = "The authority of the asset")]
    #[account(3, name="holder", desc = "The holder of the asset")]
    #[account(4, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(5, name="system_program", desc = "The system program")]
    Create(Metadata),

    /// Create a new asset.
    #[account(0, writable, name="asset", desc = "Asset account (pda of `['asset', mold pubkey]`)")]
    #[account(1, signer, name="mold", desc = "Address to derive the PDA from")]
    #[account(2, optional, name="buffer", desc = "Extension data buffer (pda of `['buffer', mold pubkey]`)")]
    #[account(3, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(4, name="system_program", desc = "The system program")]
    Initialize(Extension),
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Metadata {
    /// Some description for arg1.
    pub name: String,
    /// Some description for arg2.
    pub symbol: String,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Extension {
    /// Extension type to initialize.
    pub extension_type: ExtensionType,

    /// Extension data.
    pub data: Option<Vec<u8>>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Buffer {
    /// Indicates whether to overwrite the buffer or not.
    pub overwrite: bool,

    /// Extension data.
    pub data: Vec<u8>,
}
