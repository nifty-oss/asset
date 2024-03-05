//! Nifty Asset Types defines the types that are used to represent assets on-chain.
//!
//! - **Constraints** - these are types to define constraints when manipulating assets. They
//!   can be used to restrict the accounts that can hold, receive or send assets.
//!
//! - **Extensions** - these are types that provide additional data that can be attached to an asset.
//!   They can be used to store more information about an asset on-chain or extends their behaviour.
//!
//! - **State** - these are the types represeting the account that store the state of an asset on-chain.

pub mod constraints;
pub mod error;
pub mod extensions;
pub mod state;

/// Re-export for downstream crates.
pub mod podded {
    pub use podded::*;
}
