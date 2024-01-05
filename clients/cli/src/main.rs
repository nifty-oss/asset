use anyhow::Result;
use clap::Parser;

use nifty_cli::{
    args::{Args, Commands},
    commands::{handle_first, FirstArgs},
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::First { value } => handle_first(FirstArgs {
            keypair_path,
            rpc_url,
            value,
        }),
    }
}
