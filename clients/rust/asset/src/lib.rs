mod generated;
mod hooked;
mod impls;
mod mint;

use accounts::InternalAsset;
pub use generated::programs::ASSET_ID as ID;
pub use generated::*;
pub use hooked::*;
pub use mint::*;

// Re-export the "internal" `Asset` type.
pub type Asset = InternalAsset;

// Re-export for downstream crates.
pub mod solana_program {
    pub use solana_program::*;
}

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
