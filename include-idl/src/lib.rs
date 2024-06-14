#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "parse")]
pub use parse::parse_idl_from_program_binary;

#[cfg(feature = "shrink")]
mod shrink;
#[cfg(feature = "shrink")]
pub use shrink::compress_idl;

#[macro_export]
macro_rules! include_idl {
    ($s:expr) => {
        #[cfg_attr(target_arch = "sbf", link_section = ".solana.idl")]
        #[allow(dead_code)]
        #[no_mangle]
        pub static IDL_BYTES: &[u8] = include_bytes!($s);
    };
}
