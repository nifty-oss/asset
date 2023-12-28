use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum DASError {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,
    /// 1 - Error deserializing account
    #[error("Error deserializing account")]
    DeserializationError,
    /// 2 - Error serializing account
    #[error("Error serializing account")]
    SerializationError,
    /// 3 - Asset already initialized
    #[error("Asset already initialized")]
    AlreadyInitialized,
    /// 4 - Missing signer
    #[error("Missing signer")]
    MissingSigner,
    /// 4 - Missing extension data
    #[error("Missing extension data")]
    MissingExtensionData,
    /// 5 - Invalid account length
    #[error("Invalid account length")]
    InvalidAccountLength,
}

impl PrintProgramError for DASError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<DASError> for ProgramError {
    fn from(e: DASError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for DASError {
    fn type_of() -> &'static str {
        "Digital Asset Standard Error"
    }
}

#[macro_export]
macro_rules! err {
    ( $error:expr ) => {{
        Err($error.into())
    }};
}
