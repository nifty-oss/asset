//! This crate provides a way to include IDL files in a program binary.

pub mod parse;

#[cfg(feature = "shrink")]
mod shrink;

#[cfg(feature = "shrink")]
pub use shrink::compress_idl;

/// Include an IDL file in the program binary.
///
/// This macro creates two ELF sections in the program binary:
/// - `.idl.type` contains the type of the IDL file.
/// - `.idl.data` contains the IDL file itself.
///
/// In general you should use this macro in conbination with a `build.rs` build script
/// that compresses the IDL file to reduce the final size of the program binary.
///
/// # Arguments
///
/// This macro takes two arguments:
///
/// - `type`: The type of the IDL file. This should be one of the variants of the [`IdlType``] enum.
/// - `file`: The path to the IDL file.
///
/// # Example
///
/// Include the following in your `lib.rs` file:
///
/// ```ignore
/// include_idl!(IdlType::Codama, concat!(env!("OUT_DIR"), "/codama.idl.zip"));
/// ```
#[macro_export]
macro_rules! include_idl {
    ($type:path, $file:expr) => {
        #[cfg_attr(
            any(target_arch = "sbf", target_arch = "bpf"),
            link_section = ".idl.type"
        )]
        #[allow(dead_code)]
        #[no_mangle]
        pub static IDL_TYPE: &[u8] = $type.as_str().as_bytes();

        #[cfg_attr(
            any(target_arch = "sbf", target_arch = "bpf"),
            link_section = ".idl.data"
        )]
        #[allow(dead_code)]
        #[no_mangle]
        pub static IDL_BYTES: &[u8] = include_bytes!($file);
    };
}
