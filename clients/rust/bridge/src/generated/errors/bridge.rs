//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum BridgeError {
    /// 0 (0x0) - Invalid mint account
    #[error("Invalid mint account")]
    InvalidMint,
    /// 1 (0x1) - Invalid authority account
    #[error("Invalid authority account")]
    InvalidAuthority,
}

impl solana_program::program_error::PrintProgramError for BridgeError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
