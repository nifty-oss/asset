use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateArgs {
    pub is_collection: bool,
}

#[rustfmt::skip]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
pub enum Instruction {
    /// Creates a proxied asset.
    #[account(0, writable, name="asset", desc = "The proxied asset (seeds: `[stub]`)")]
    #[account(1, signer, name="stub", desc = "The ephemeral stub to derive the address of the asset")]
    #[account(2, name="owner", desc = "The owner of the asset")]
    #[account(3, writable, signer, optional, name="payer", desc = "The account paying for the storage fees")]
    #[account(4, optional, name="system_program", desc = "System program")]
    #[account(5, name="nifty_asset_program", desc = "Nifty Asset program")]
    Create,
}
