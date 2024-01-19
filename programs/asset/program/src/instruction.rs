use borsh::{BorshDeserialize, BorshSerialize};
use nifty_asset_types::{
    extensions::ExtensionType,
    state::{DelegateRole, Standard},
};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum Instruction {
    /*
    /// Closes an extension data buffer.
    /// 
    /// You can only close the asset account if it has not being created.
    #[account(0, signer, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    Close,
    */

    /// Burns an asset.
    #[account(0, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, writable, name="signer", desc = "The holder or burn delegate of the asset")]
    #[account(2, optional, writable, name="recipient", desc = "The account receiving refunded rent")]
    Burn,

    /// Creates a new asset.
    #[account(0, signer, writable, name="asset", desc = "Asset account")]
    #[account(1, name="authority", desc = "The authority of the asset")]
    #[account(2, name="holder", desc = "The holder of the asset")]
    #[account(3, optional, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(4, optional, name="system_program", desc = "The system program")]
    Create(Metadata),

    /// Approves a delegate to manage an asset.
    #[account(0, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, name="holder", desc = "The holder of the asset")]
    #[account(2, name="delegate", desc = "The delegate account")]
    Delegate(Vec<DelegateRole>),

    /// Allocates an extension.
    #[account(0, signer, writable, name="asset", desc = "Asset account")]
    #[account(1, optional, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, optional, name="system_program", desc = "The system program")]
    Allocate(Extension),

    /// Locks an asset.
    #[account(0, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, name="authority", desc = "Delegate or holder account")]
    Lock,

    /// Transfers ownership of the aseet to a new public key.
    #[account(0, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, name="signer", desc = "Current holder of the asset or transfer delegate")]
    #[account(2, name="recipient", desc = "The recipient of the asset")]
    Transfer,

    /// Unlocks an asset.
    #[account(0, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, name="authority", desc = "Delegate ot holder account")]
    Unlock,

    /// Writes data to an extension.
    #[account(0, signer, writable, name="asset", desc = "Asset account")]
    #[account(1, signer, writable, name="payer", desc = "The account paying for the storage fees")]
    #[account(2, name="system_program", desc = "The system program")]
    Write(Data),
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Metadata {
    /// Name of the asset.
    pub name: String,

    /// Indicates the standard of an asset.
    pub standard: Standard,

    /// Indicates whether the asset is mutable or not.
    pub mutable: bool,
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
