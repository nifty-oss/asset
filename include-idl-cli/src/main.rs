use flate2::bufread::ZlibDecoder;
use goblin::elf::Elf;
use goblin::error::Result;
use serde_json::Value;
use std::{io::Read, path::PathBuf};

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

pub fn parse_idl_from_program_binary(buffer: &[u8]) -> Result<Value> {
    let elf = Elf::parse(&buffer)?;

    // Iterate over section headers and print information
    for sh in &elf.section_headers {
        let name = elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("<invalid>");
        if name == ".solana.idl" {
            // Get offset of .solana.idl section data
            let offset = sh.sh_offset as usize;

            // Get offset & size of the compressed IDL bytes
            let _data_loc = &buffer[offset + 4..offset + 8];
            let data_loc = u32::from_le_bytes(_data_loc.try_into().unwrap());
            let _data_size = &buffer[offset + 8..offset + 16];
            let data_size = u64::from_le_bytes(_data_size.try_into().unwrap());

            let compressed_data =
                &buffer[data_loc as usize..(data_loc as u64 + data_size) as usize];
            let mut d = ZlibDecoder::new(compressed_data);
            let mut decompressed_data = Vec::new();
            d.read_to_end(&mut decompressed_data).unwrap();

            let json: Value = serde_json::from_slice(&decompressed_data).unwrap();
            return Ok(json);
        }
    }
    Err(goblin::error::Error::Malformed(
        "Could not find .solana.idl section".to_string(),
    ))
}
