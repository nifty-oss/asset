mod generated;
mod mint;
pub mod extensions {
    pub use nifty_asset_types::extensions::*;
}
pub mod state {
    pub use nifty_asset_types::state::*;
}

pub use generated::programs::ASSET_ID as ID;
pub use generated::*;

pub use mint::*;
pub use nifty_asset_types::podded::ZeroCopy;
