use std::path::PathBuf;

use clap::{Parser, Subcommand};
use solana_program::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the keypair file.
    #[arg(short, long, global = true)]
    pub keypair_path: Option<PathBuf>,

    /// RPC URL for the Solana cluster.
    #[arg(short, long, global = true)]
    pub rpc_url: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    /// Burn an asset.
    Burn {
        /// The asset to burn.
        asset: Pubkey,

        /// The recipient to receive reclaimed rent. Defaults to the signer.
        recipient: Option<Pubkey>,
    },
    Mint {
        asset_file_path: PathBuf,
    },
    /// Create an asset.
    Create {
        /// The name of the asset.
        #[arg(short, long)]
        name: String,

        /// Path to the mint keypair file
        #[arg(short, long)]
        asset_keypair_path: Option<PathBuf>,

        /// Create the asset as immutable.
        #[arg(long)]
        immutable: bool,

        /// Owner of the created asset, defaults to authority pubkey.
        #[arg(short, long)]
        owner: Option<Pubkey>,
    },
    /// Get an asset account's data and decode it.
    Decode {
        /// The asset to decode.
        asset: Pubkey,

        /// The field to decode.
        /// If not specified, the entire asset will be decoded.
        #[arg(short, long)]
        field: Option<String>,
    },
    /// Set a delegate on an asset with specific roles.
    Delegate {
        /// The asset to delegate.
        asset: Pubkey,

        /// The address to delegate to.
        delegate: Pubkey,

        /// The role for the delegate to have: "burn", "lock", "transfer".
        /// Specify each one separately: --role burn --role lock --role transfer
        #[arg(short = 'R', long)]
        role: Vec<String>,
    },
    /// Lock an asset, preventing any actions to be performed on it.
    Lock {
        /// The asset to lock.
        asset: Pubkey,

        /// Path to the authority keypair file. Defaults to the signer.
        authority_keypair_path: Option<PathBuf>,
    },
    /// Transfer an asset to a new holder.
    Transfer {
        /// The asset to transfer.
        asset: Pubkey,

        /// The recipient of the asset.
        recipient: Pubkey,
    },
    /// Unlock an asset, allowing actions to be performed on it.
    Unlock {
        /// The asset to unlock.
        asset: Pubkey,

        /// Path to the authority keypair file. Defaults to the signer.
        authority_keypair_path: Option<PathBuf>,
    },
}
