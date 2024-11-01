use clap::{Error, Parser, Subcommand};
use solana_include_idl::parse::parse_idl_from_program_binary;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Read IDL from a solana program binary.
    Parse {
        /// Path to the program binary.
        path: PathBuf,
    },
}

pub fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Parse { path }) => {
            let buffer = std::fs::read(path).expect("Could not read file.");
            if let Ok((idl_type, idl_data)) = parse_idl_from_program_binary(&buffer) {
                println!("Program IDL ({idl_type})");
                println!("============================");
                println!("\n{idl_data:#?}");
            } else {
                println!("Could not find IDL in program binary");
            }
        }
        None => {}
    }
    Ok(())
}
