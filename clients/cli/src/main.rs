use anyhow::Result;
use clap::Parser;

use nifty_cli::{
    args::{Args, Commands},
    commands::*,
};

#[tokio::main]
async fn main() -> Result<()> {
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
        Commands::Decode { asset, field, raw } => handle_decode(DecodeArgs {
            rpc_url,
            asset,
            field,
            raw,
        }),
        Commands::Approve {
            asset,
            delegate,
            role,
        } => handle_approve(ApproveArgs {
            keypair_path,
            rpc_url,
            asset,
            delegate,
            role,
        }),
        Commands::Lock {
            asset,
            signer_keypair_path,
        } => handle_lock(LockArgs {
            keypair_path,
            rpc_url,
            asset,
            signer_keypair_path,
        }),
        Commands::Mint { asset_file_path } => {
            handle_mint(MintArgs {
                keypair_path,
                rpc_url,
                asset_file_path,
            })
            .await
        }
        Commands::MintBatch { asset_files_dir } => {
            handle_mint_batch(MintBatchArgs {
                keypair_path,
                rpc_url,
                asset_files_dir,
            })
            .await
        }
        Commands::Revoke { asset, role, all } => handle_revoke(RevokeArgs {
            keypair_path,
            rpc_url,
            asset,
            role,
            all,
        }),
        Commands::Transfer { asset, recipient } => handle_transfer(TransferArgs {
            keypair_path,
            rpc_url,
            asset,
            recipient,
        }),
        Commands::Unlock {
            asset,
            signer_keypair_path,
        } => handle_unlock(UnlockArgs {
            keypair_path,
            rpc_url,
            asset,
            signer_keypair_path,
        }),
    }
}
