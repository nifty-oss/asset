mod generated;
mod hooked;
mod impls;
mod mint;

pub use generated::programs::ASSET_ID as ID;
pub use generated::*;
pub use hooked::*;
pub use mint::*;

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
