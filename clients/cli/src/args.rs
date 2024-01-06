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
    Burn {
        /// The asset to burn.
        asset: Pubkey,

        /// The recipient to receive reclaimed rent. Defaults to the signer.
        recipient: Option<Pubkey>,
    },
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
    Decode {
        asset: Pubkey,
    },
    Transfer {
        /// The asset to transfer.
        asset: Pubkey,

        /// The recipient of the asset.
        recipient: Pubkey,
    },
}
