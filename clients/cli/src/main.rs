use anyhow::Result;
use clap::Parser;

use nifty_cli::{
    args::{Args, Commands},
    commands::*,
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=error");

    let args = Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::Burn { asset, recipient } => handle_burn(BurnArgs {
            keypair_path,
            rpc_url,
            asset,
            recipient,
        }),
        Commands::Create {
            name,
            asset_keypair_path,
            immutable,
            owner,
        } => handle_create(CreateArgs {
            keypair_path,
            rpc_url,
            name,
            asset_keypair_path,
            immutable,
            owner,
        }),
        Commands::Decode { asset, field } => handle_decode(DecodeArgs {
            rpc_url,
            asset,
            field,
        }),
        Commands::Delegate {
            asset,
            delegate,
            role,
        } => handle_delegate(DelegateArgs {
            keypair_path,
            rpc_url,
            asset,
            delegate,
            role,
        }),
        Commands::Lock {
            asset,
            authority_keypair_path,
        } => handle_lock(LockArgs {
            keypair_path,
            rpc_url,
            asset,
            authority_keypair_path,
        }),
        Commands::Transfer { asset, recipient } => handle_transfer(TransferArgs {
            keypair_path,
            rpc_url,
            asset,
            recipient,
        }),
        Commands::Unlock {
            asset,
            authority_keypair_path,
        } => handle_unlock(UnlockArgs {
            keypair_path,
            rpc_url,
            asset,
            authority_keypair_path,
        }),
    }
}
