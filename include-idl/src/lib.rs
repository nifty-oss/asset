pub mod parse;

#[cfg(feature = "shrink")]
mod shrink;

#[cfg(feature = "shrink")]
pub use shrink::compress_idl;

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
