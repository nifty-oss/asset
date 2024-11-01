# `solana-include-idl`

A collection of macro and helpers to manage IDLs stored on the program binary.

IDL files describe the structure and API of a Solana program, facilitating easier integration and interaction with various client applications. This crate automates the task of publishing the IDL file by storing it as a separate ELF section within the program binary.
 
## Usage

The crate provides a macro that includes the type and contents of the IDL file in separate ELF sections on the program binary.

* `.idl.type` contains the type of the IDL file.
* `.idl.data` contains the IDL file itself.

The macro takes two arguments:

* `type`: The type of the IDL file. This should be one of the variants of the `IdlType` enum (e.g., `IdlType::Anchor` or `IdlType::Codama`).
* `file`: The path to the IDL file.

```rust
use include_idl::{include_idl, parse::IdlType};

include_idl!(IdlType::Codama, concat!(env!("OUT_DIR"), "/codama.idl.zip"));
```

In general, the macro is used in combination with a `build.rs` build script to compress the IDL file, reducing the space required on program binary.

To specify a build script, add an `build = "build.rs"` entry on your `Cargo.toml` file under the `[package]` section. Below is a `build.rs` example file that compresses an existing IDL file.

```rust
use std::env;
use std::path::PathBuf;
use include_idl::compress_idl;

fn main() {
    // Get the IDL path.
    let idl_path = PathBuf::from("../api").join("idl.json");
    // Compress the IDL file to a zip file.
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = PathBuf::from(out_dir).join("codama.idl.zip");

    compress_idl(&idl_path, &dest_path);
}
```

### Generating an IDL

If you are using [Anchor](https://www.anchor-lang.com), this step is alredy done for your. If you are writing a native Solana program or using a framework that does not export and IDL, you can use [Codama](https://github.com/codama-idl/codama?tab=readme-ov-file#from-program-to-codama).