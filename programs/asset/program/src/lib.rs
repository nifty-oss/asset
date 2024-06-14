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

#[cfg(not(feature = "no-entrypoint"))]
use include_idl::include_idl;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
include_idl!(concat!(env!("OUT_DIR"), "/idl.json.zip"));

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    // Required fields
    name: "Nifty Asset",
    project_url: "https://nifty-oss.org",
    contacts: "email:maintainers@nifty-oss.org,link:https://twitter.com/nifty_oss,link:https://discord.gg/Ctf52swtH3",
    policy: "https://github.com/nifty-oss/asset/blob/main/SECURITY.md",

    // Optional Fields
    source_code: "https://github.com/nifty-oss/asset"
}

solana_program::declare_id!("AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73");
