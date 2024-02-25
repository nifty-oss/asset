//! Nifty Asset Types defines the types that are used to represent assets on-chain.
//!
//! - **Constraints** - Defines the constraints that are used to validate the asset.
//!
//! - **Extensions** - Defines the constraints that are used to validate the asset.
//!
//! - **State** - Defines the constraints that are used to validate the asset.

pub mod constraints;
pub mod error;
pub mod extensions;
pub mod state;

/// Re-export for downstream crates.
pub mod podded {
    pub use podded::*;
}

/// Re-export for downstream crates.
pub mod solana_program {
    pub use solana_program::*;
}
