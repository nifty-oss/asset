mod generated;
pub mod extensions {
    pub use nifty_asset_types::extensions::*;
}
pub mod state {
    pub use nifty_asset_types::state::*;
}

pub use generated::programs::ASSET_ID as ID;
pub use generated::*;
pub use nifty_asset_types::ZeroCopy;
