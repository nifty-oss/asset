use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateArgs {
    pub is_collection: bool,
    pub max_collection_size: Option<u64>,
}

#[rustfmt::skip]
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
pub enum Instruction {
    /// Bridge between non-fungible token and asset.
    #[account(0, writable, name="asset", desc="Asset account of the mint (pda of `['nifty::bridge::asset', mint pubkey]`)")]
    #[account(1, writable, name="vault", desc="Bridge account for the asset (pda of `['nifty::bridge::vault', mint pubkey]`)")]
    #[account(2, signer, name="owner", desc="Token owner account")]
    #[account(3, writable, name="token", desc="Token account")]
    #[account(4, name="mint", desc="Mint account of the token")]
    #[account(5, writable, name="metadata", desc="Metadata account of the mint")]
    #[account(6, name="master_edition", desc="Master Edition of the mint")]
    #[account(7, optional, writable, name="token_record", desc="Owner token record account")]
    #[account(8, writable, name="vault_token", desc="Vault token account")]
    #[account(9, optional, writable, name="vault_token_record", desc="Vault token record account")]
    #[account(10, signer, writable, name="payer", desc="The account paying for the storage fees")]
    #[account(11, name="nifty_asset_program", desc = "Nifty Asset program")]
    #[account(12, name="token_metadata_program", desc = "Metaplex Token Metadata program")]
    #[account(13, name="system_program", desc = "System program")]
    #[account(14, name="sysvar_instructions", desc = "Instructions sysvar account")]
    #[account(15, name="spl_token_program", desc="SPL Token program")]
    #[account(16, name="spl_ata_program", desc="SPL ATA program")]
    #[account(17, optional, name="authorization_rules_program", desc="Token Auth Rules program")]
    #[account(18, optional, name="authorization_rules", desc="Token Auth Rules account")]
    #[account(19, optional, name="group_asset", desc="Group asset account")]
    Bridge,

    /// Create an asset on the bridge from an existing non-fungible token.
    #[account(0, writable, name="asset", desc="Asset account of the mint (pda of `['nifty::bridge::asset', mint pubkey]`)")]
    #[account(1, writable, name="vault", desc="Bridge account for the asset (pda of `['nifty::bridge::vault', mint pubkey]`)")]
    #[account(2, name="mint", desc="Mint account of the token")]
    #[account(3, name="metadata", desc="Metadata account of the collection")]
    #[account(4, optional_signer, name="update_authority", desc="Update authority of the metadata")]
    #[account(5, optional, name="collection", desc="Asset account of the collection (pda of `['nifty::bridge::asset', collection mint pubkey]`)")]
    #[account(6, signer, writable, name="payer", desc="The account paying for the storage fees")]
    #[account(7, name="system_program", desc = "System program")]
    #[account(8, name="nifty_asset_program", desc = "Nifty Asset program")]
    Create(CreateArgs),
}
