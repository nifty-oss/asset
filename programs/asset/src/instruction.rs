use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

use crate::extensions::ExtensionType;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    /*
    /// Closes an extension data buffer.
    /// 
    /// You can only close the asset account if it has not being created.
    #[account(0, writable, name="buffer", desc = "Data buffer (pda of `['buffer', authority pubkey]`)")]
    #[account(1, signer, name="authority", desc = "Authority of the buffer")]
    #[account(2, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    Close,
    */

    /// Create a new asset.
    #[account(0, writable, name="asset", desc = "Asset account (pda of `['asset', canvas pubkey]`)")]
    #[account(1, signer, name="canvas", desc = "Address to derive the PDA from")]
    #[account(2, signer, name="authority", desc = "The authority of the asset")]
    #[account(3, name="holder", desc = "The holder of the asset")]
    #[account(4, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(5, name="system_program", desc = "The system program")]
    Create(Metadata),

    /// Create a new asset.
    #[account(0, writable, name="asset", desc = "Asset account (pda of `['asset', canvas pubkey]`)")]
    #[account(1, signer, name="canvas", desc = "Address to derive the PDA from")]
    #[account(2, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, name="system_program", desc = "The system program")]
    Initialize(Extension),

    /// Allocate space for an extension data buffer.
    #[account(0, writable, name="asset", desc = "Asset account (pda of `['asset', canvas pubkey]`)")]
    #[account(1, signer, name="canvas", desc = "Address to derive the PDA from")]
    #[account(2, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(3, name="system_program", desc = "The system program")]
    Write(Data),
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

    /// Total length of the extension data.
    pub length: u32,

    /// Extension data.
    pub data: Option<Vec<u8>>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Data {
    /// Indicates whether to overwrite the buffer or not.
    pub overwrite: bool,

    /// Extension data.
    pub bytes: Vec<u8>,
}
