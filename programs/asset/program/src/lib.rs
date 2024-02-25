//! Nifty Asset is a free, open standard that describes how to build non-fungible or
//! unique tokens (digital assets) on the Solana blockchain.
//!
//! A digital asset is viewed as unique *slab* of bytes on the blockchain identified by
//! an address. The contents of an asset is highly flexible: it can represent assets fully
//! stored on-chain or assets that have external "pointers". This is achieved by using a
//! set of extensions that can be added to an asset at creation.

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod utils;

pub use solana_program;

solana_program::declare_id!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73");
