use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum BridgeError {
    /// 0 - Invalid mint account
    #[error("Invalid mint account")]
    InvalidMint,

    /// 1 - Invalid authority account
    #[error("Invalid authority account")]
    InvalidAuthority,
}

impl PrintProgramError for BridgeError {
    fn print<E>(&self) {
        msg!("⛔️ {} ({:?})", &self.to_string(), &self);
    }
}

impl From<BridgeError> for ProgramError {
    fn from(e: BridgeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for BridgeError {
    fn type_of() -> &'static str {
        "nifty::bridge"
    }
}

#[macro_export]
macro_rules! err {
    ( $error:expr ) => {{
        Err($error.into())
    }};
    ( $error:expr, $msg:expr ) => {{
        solana_program::msg!("[ERROR] {}", $msg);
        Err($error.into())
    }};
    ( $error:expr, $msg:literal, $($args:tt)+ ) => {{
        err!($error, &format!($msg, $($args)+))
    }};
}
