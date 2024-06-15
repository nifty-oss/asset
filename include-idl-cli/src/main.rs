use std::path::PathBuf;

use goblin::error::Result;
use include_idl::parse_idl_from_program_binary;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Parse {
        /// Read IDL from a solana program binary
        path: PathBuf,
    },
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { path }) => {
            let buffer = std::fs::read(path).expect("Could not read file.");
            let idl = parse_idl_from_program_binary(&buffer)?;
            println!("        Program IDL");
            println!("============================");
            println!("{}", idl);
        }
        None => {}
    }
    Ok(())
}
