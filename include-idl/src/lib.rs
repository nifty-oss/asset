mod parse;
mod shrink;

#[cfg(feature = "shrink")]
pub use shrink::compress_idl;

#[cfg(feature = "parse")]
pub use parse::parse_idl_from_program_binary;

#[macro_export]
macro_rules! include_idl {
    () => {
        #[cfg(target_arch = "bpf")]
        #[link_section = ".solana.idl"]
        #[allow(dead_code)]
        #[no_mangle]
        pub static IDL_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/idl.json.zip"));
    };
}
