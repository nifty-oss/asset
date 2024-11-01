use std::env;
use std::path::PathBuf;
use std::process::Command;

use solana_include_idl::compress_idl;

fn main() {
    // Run shank to generate the IDL
    let _output = Command::new("pnpm")
        .arg("generate:idls")
        .output()
        .expect("Failed to run Shank");

    let _output = Command::new("pnpm")
        .arg("generate:clients")
        .output()
        .expect("Failed to create the Codama IDLs");

    // Get the IDL path
    let idl_path = PathBuf::from("../../../idls").join("asset-program.codama.json");

    // Concat output path of compressed IDL
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("codama.idl.zip");

    compress_idl(&idl_path, &dest_path);
}
