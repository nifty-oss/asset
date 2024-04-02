mod generated;
mod interface;

pub use interface::*;
use solana_program::pubkey::Pubkey;

solana_program::declare_id!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73");

// Redefine constant to change its visibility.
pub const INTERFACE_ID: Pubkey = generated::INTERFACE_ID;

// Re-export nifty_asset_types for convenience

pub mod constraints {
    pub use nifty_asset_types::constraints::*;
}
pub mod extensions {
    pub use nifty_asset_types::extensions::*;
}
pub mod state {
    pub use nifty_asset_types::state::*;
}
pub use nifty_asset_types::podded::ZeroCopy;
