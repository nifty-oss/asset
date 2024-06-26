use std::{io::ErrorKind, path::PathBuf, str::FromStr};

// use goblin::error::Result;
use include_idl::parse::{parse_idl_from_program_binary, IdlType};

use clap::{Error, Parser, Subcommand};

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
        idl_type: IdlType
    },
}

// This example uses ArgEnum, so this might not be necessary.

pub fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { path, idl_type }) => {
            let buffer = std::fs::read(path).expect("Could not read file.");
            if let Ok(idl) = parse_idl_from_program_binary(&buffer, idl_type.clone()) {
                println!("        Program IDL");
                println!("============================");
                println!("{}", idl);
            } else {
                println!("Could not find {:?} IDL in program binary", idl_type);
            }
        }
        None => {}
    }
    Ok(())
}
